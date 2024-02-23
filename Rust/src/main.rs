// N queens problem (Rust version)
// Copyleft 2022 Vikman - All rights revoked.
// July 31, 2022

mod chess;
mod solver;

use std::env;
use std::process::exit;
use std::time::{Instant,Duration};
use chess::Chess;
use solver::{Solver,SerialSolver, ParallelSolver};

const DEFAULT_SIZE: usize = 100;

fn main() {
    let (size, parallel, testing) = parse_args();
    let chess = Chess::new(size);

    let solver: &mut dyn Solver;

    let mut serial_solver: SerialSolver;
    let mut parallel_solver: ParallelSolver;

    if parallel {
        parallel_solver = ParallelSolver::new(chess);
        solver = &mut parallel_solver;
    } else {
        serial_solver = SerialSolver::new(chess);
        solver = &mut serial_solver;
    }

    let now = Instant::now();

    if !solver.solve() {
        panic!("Cannot resolve the problem");
    }

    print_results(solver, now.elapsed(), testing);

}

/// Parse arguments. Returns the chess size and the parallel mode
fn parse_args() -> (usize, bool, bool) {
    let mut size = DEFAULT_SIZE;
    let mut parallel = false;
    let mut testing = false;

    for arg in env::args() {
        if arg == "-h" || arg == "--help" {
            println!("Syntax: queens [-p] <NUMBER>");
            exit(0);
        } else if arg == "-p" || arg == "--parallel" {
            parallel = true;
        } else if arg == "-test" {
            testing = true;
        } else if let Ok(v) = arg.parse::<usize>() {
            size = v;
        }
    }

    (size, parallel, testing)
}

/// Print algorithm results into stdout and stderr.
fn print_results(solver: &dyn Solver, duration: Duration, testing: bool) {
    if testing {
        println!("{}\t{}\t{}", solver.trials(), solver.discards(), duration.as_micros());
    } else {
        print!("{}", solver.chess());
        eprintln!("Trials:      {}", solver.trials());
        eprintln!("Discards:    {}", solver.discards());
        eprintln!("Time:        {:.3} ms.", duration.as_millis());
        eprintln!("Performance: {:.3} steps/μs.", solver.trials() as f64 / duration.as_micros() as f64);
        eprintln!("             {:.3} discards/μs.", solver.discards() as f64 / duration.as_micros() as f64);
    }
}
