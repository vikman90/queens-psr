// N queens problem (Rust version)
// Copyleft 2022 Vikman - All rights revoked.
// July 31, 2022

mod chess;
mod solver;

use std::env;
use std::process::exit;
use std::time::{Instant,Duration};
use chess::Chess;
use solver::Solver;

const DEFAULT_SIZE: usize = 100;

fn main() {
    let size = parse_args();
    let chess = Chess::new(size);
    let mut solver = Solver::new(chess);

    let now = Instant::now();

    if !solver.solve() {
        panic!("Cannot resolve the problem");
    }

    print_results(&solver, now.elapsed());

}

/// Parse arguments. Returns the chess size.
fn parse_args() -> usize {
    let mut size = DEFAULT_SIZE;

    for arg in env::args() {
        if arg == "-h" || arg == "--help" {
            println!("Syntax: queens <NUMBER>");
            exit(0);
        } else if let Ok(v) = arg.parse::<usize>() {
            size = v;
        }
    }

    size
}

/// Print algorithm results into stdout and stderr.
fn print_results(solver: &Solver, duration: Duration) {
    print!("{}", solver.chess());
    eprintln!("Trials:      {}", solver.trials());
    eprintln!("Discards:    {}", solver.discards());
    eprintln!("Time:        {:.3} ms.", duration.as_millis());
    eprintln!("Performance: {:.3} steps/μs.", solver.trials() as f64 / duration.as_micros() as f64);
    eprintln!("             {:.3} discards/μs.", solver.discards() as f64 / duration.as_micros() as f64);
}
