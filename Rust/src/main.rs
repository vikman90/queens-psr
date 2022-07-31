// N queens problem (Rust version)
// Copyleft 2022 Vikman - All rights revoked.
// July 31, 2022

mod chess;

use std::env;
use std::process::exit;
use std::time::Instant;
use chess::Chess;

const DEFAULT_SIZE: usize = 100;

fn main() {
    let size = parse_args();
    let mut chess = Chess::new(size);

    let now = Instant::now();
    chess.solve();
    let duration = now.elapsed();

    print!("{}", chess);
    eprintln!("Trials:      {}", chess.trials());
    eprintln!("Discards:    {}", chess.discards());
    eprintln!("Time:        {:.3} ms.", duration.as_millis());
    eprintln!("Performance: {:.3} steps/μs.", chess.trials() as f64 / duration.as_micros() as f64);
    eprintln!("             {:.3} discards/μs.", chess.discards() as f64 / duration.as_micros() as f64);
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
