// N queens problem (Rust version)
// Copyleft 2022 Vikman - All rights revoked.
// July 31, 2022

use std::fmt;
use std::ops::{Index,IndexMut};
use rand::{thread_rng, seq::SliceRandom};

#[derive(Clone)]
pub struct ChessRow {
    /// Queens' positions (set of candidate positions).
    candidates: Vec<bool>,
    /// Number of available values.
    count: usize,
}

pub struct Chess {
    /// Chess rows.
    queens: Vec<ChessRow>,
}

impl ChessRow {
    /// Constructor.
    fn new(size: usize) -> Self {
        Self { candidates: vec![true; size], count: size }
    }

    /// Get number of candidates.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Clear a row (empty candidates).
    pub fn clear(&mut self) {
        self.candidates.fill(false);
        self.count = 0;
    }

    /// Push a candidate value back to a row.
    pub fn push(&mut self, value: usize) {
        if !self.candidates[value] {
            self.candidates[value] = true;
            self.count += 1;
        }
    }

    /// Remove a candidate from a row.
    pub fn remove(&mut self, value: usize) -> bool {
        if self.candidates[value] {
            self.candidates[value] = false;
            self.count -= 1;
            true
        } else {
            false
        }
    }

    /// Get the _only_ value that is set in the array.
    pub fn get_value(&self) -> usize {
        self.candidates.iter().position(|&s| s).unwrap()
    }

    /// Select all candidate values.
    pub fn as_vec(&self) -> Vec<usize> {
        let mut values: Vec<usize> = (0..self.candidates.len()).filter(|&s| self.candidates[s]).collect();
        values.shuffle(&mut thread_rng());
        values
    }
}

impl Index<usize> for ChessRow {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.candidates[index]
    }
}

impl Chess {
    /// Constructor.
    pub fn new(size: usize) -> Self {
        Self {
            queens: vec![ChessRow::new(size); size],
        }
    }

    /// Get the length of the chess.
    pub fn len(&self) -> usize {
        self.queens.len()
    }
}

impl Index<usize> for Chess {
    type Output = ChessRow;

    fn index(&self, i: usize) -> &Self::Output {
        &self.queens[i]
    }
}

impl IndexMut<usize> for Chess {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.queens[i]
    }
}

impl fmt::Display for Chess {
    /// Representation of the chessboard.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.queens.len() {
            let row = &self.queens[i];

            if row.count() == 1 {
                if let Err(x) = writeln!(f, "Queen {}: square {}", i + 1, row.get_value()) {
                    return Err(x);
                }
            } else {
                if let Err(x) = writeln!(f, "Queen {} not solved.", i + 1) {
                    return Err(x);
                }
            }
        }

        fmt::Result::Ok(())
    }
}
