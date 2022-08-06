// N queens problem (Rust version)
// Copyleft 2022 Vikman - All rights revoked.
// August 4, 2022

use std::sync::{mpsc, Mutex, Arc};
use mpsc::{Sender, Receiver};
use std::thread::{self, JoinHandle};

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
    /// Threads.
    threads: Vec<JoinHandle<()>>,
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
            discard_ops: 0,
            threads: Vec::new(),
        }
    }

    /// Send a new chess to a thread.
    fn send_work(&mut self, work_tx: &Sender<(SerialSolver, usize)>) -> bool {
        match self.values.pop() {
            Some(value) => {
                let solver = self.root.clone();
                work_tx.send((solver, value)).unwrap();
                self.pending_results += 1;
                true
            },

            None => false,
        }
    }

    /// Process result from a thread.
    fn process_result(&mut self, solver: SerialSolver, is_solved: bool) {
        self.trial_ops += solver.trials();
        self.discard_ops += solver.discards();
        self.pending_results -= 1;

        if is_solved {
            self.root = solver;
        }
    }

    fn start_threads(&mut self) -> (Sender<(SerialSolver, usize)>, Receiver<Message>) {
        let (work_tx, work_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let work_rx_lock = Arc::new(Mutex::new(work_rx));

        self.threads = (0..num_cpus::get()).map(|_| {
            let rx = Arc::clone(&work_rx_lock);
            let tx = result_tx.clone();

            thread::spawn(move || { thread_run(rx, tx);})
        }).collect();

        (work_tx, result_rx)
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
        let mut solved = false;

        {
            let (work_tx, result_rx) = self.start_threads();

            loop {
                match result_rx.recv().unwrap() {
                    Message::Ready => {
                        if !self.send_work(&work_tx) {
                            break;
                        }
                    },
                    Message::Result((solver, is_solved)) => {
                        self.process_result(solver, is_solved);

                        if is_solved {
                            solved = true;
                            break;
                        }
                    }
                }
            }

            self.root.stop();

            while self.pending_results > 0 {
                match result_rx.recv().unwrap() {
                    Message::Ready => (),
                    Message::Result((solver, is_solved)) => {
                        self.process_result(solver, is_solved);
                    }
                }
            }
        }

        while let Some(t) = self.threads.pop() {
            _ = t.join();
        }

        solved
    }
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
        let is_stopped = solver.is_stopped();

        if let Err(_) = result_tx.send(Message::Result((solver, is_solved))) {
            return;
        }

        if is_stopped {
            return;
        }
    }
}
