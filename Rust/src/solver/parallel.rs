// N queens problem (Rust version)
// Copyleft 2022 Vikman - All rights revoked.
// August 4, 2022

use std::sync::{mpsc, Mutex, Arc};
use mpsc::{Sender, Receiver, channel};
use std::thread;

use num_cpus;

use crate::solver::{Solver,SerialSolver};
use crate::chess::Chess;

enum Message {
    Ready,
    Result((SerialSolver, bool))
}

pub struct ParallelSolver {
    /// Main solver (search tree's root).
    pub root: SerialSolver,
    /// Candidate values (first row).
    values: Vec<usize>,
    // Number of threads working.
    pending_results: u32,
    /// Number of calls to assign().
    trial_ops: u64,
    /// Number of calls to discard().
    discard_ops: u64,
}

impl ParallelSolver {
    /// Constructor.
    pub fn new(chess: Chess) -> Self {
        let values = chess[0].as_vec();

        Self {
            root: SerialSolver::new(chess),
            values,
            pending_results: 0,
            trial_ops: 0,
            discard_ops: 0
        }
    }

    /// Send a new chess to a thread.
    fn send_work(&mut self, work_tx: &Sender<(SerialSolver, usize)>) -> bool {
        match self.values.pop() {
            Some(value) => {
                let solver = self.root.clone();
                work_tx.send((solver, value)).unwrap();
                self.pending_results += 1;
            },

            None => (),
        }

        if self.values.is_empty() && self.pending_results == 0 { false } else { true }
    }

    /// Process result from a thread.
    fn process_result(&mut self, solver: SerialSolver, is_solved: bool) -> bool {
        self.trial_ops += solver.trials();
        self.discard_ops += solver.discards();

        if is_solved {
            self.root = solver;
            true
        } else {
            self.pending_results -= 1;
            false
        }
    }
}

impl Solver for ParallelSolver {
    /// Get chess.
    fn chess(&self) -> &Chess {
        self.root.chess()
    }

    /// Get number of total tries of assignation.
    fn trials(&self) -> u64 {
        self.trial_ops
    }

    /// Get number of total discards.
    fn discards(&self) -> u64 {
        self.discard_ops
    }

    /// Solve queens problem.
    fn solve(&mut self) -> bool {
        let (work_tx, result_rx) = start_threads();

        loop {
            match result_rx.recv().unwrap() {
                Message::Ready => {
                    if !self.send_work(&work_tx) {
                        break;
                    }
                },
                Message::Result((solver, is_solved)) => {
                    if self.process_result(solver, is_solved) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn start_threads() -> (Sender<(SerialSolver, usize)>, Receiver<Message>) {
    let (work_tx, work_rx) = channel();
    let (result_tx, result_rx) = channel();
    let work_rx_lock = Arc::new(Mutex::new(work_rx));

    for _ in 0..num_cpus::get() {
        let rx = Arc::clone(&work_rx_lock);
        let tx = result_tx.clone();

        thread::spawn(move || { thread_run(rx, tx);});
    }

    (work_tx, result_rx)
}

fn thread_run(work_rx_lock: Arc<Mutex<Receiver<(SerialSolver, usize)>>>, result_tx: Sender<Message>) {
    loop {
        if let Err(_) = result_tx.send(Message::Ready) {
            return;
        }

        let mut solver;
        let value;

        match work_rx_lock.lock() {
            Ok(work_rx) => match work_rx.recv() {
                Ok((s, v)) => (solver, value) = (s, v),
                Err(_) => return,
            },
            Err(_) => return,
        }

        let is_solved = solver.assign(0, value) && solver.branch();

        if let Err(_) = result_tx.send(Message::Result((solver, is_solved))) {
            return;
        }
    }
}
