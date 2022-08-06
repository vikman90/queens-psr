// N queens problem (Rust version)
// Copyleft 2022 Vikman - All rights revoked.
// July 31, 2022

mod serial;
mod parallel;

use crate::chess::Chess;

pub trait Solver {
    /// Solve queens problem.
    fn solve(&mut self) -> bool;

    /// Get chess.
    fn chess(&self) -> &Chess;

    /// Get number of total tries of assignation.
    fn trials(&self) -> u64;

    /// Get number of total discards.
    fn discards(&self) -> u64;
}

pub use serial::SerialSolver;
pub use parallel::ParallelSolver;
