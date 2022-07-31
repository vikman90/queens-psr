// N queens problem (Rust version)
// Copyleft 2022 Vikman - All rights revoked.
// July 31, 2022

use std::fmt;
use std::collections::HashSet;

pub struct Chess {
    /// Number of calls to assign().
    trial_ops: u64,
    /// Number of calls to discard().
    discard_ops: u64,
    /// Queens' positions (set of candidate positions).
    queens: Vec<HashSet<usize>>,
    /// Discarded candidates (index-value).
    discarded_pairs: Vec<(usize, usize)>,
    /// Number of discards in the last assignation.
    discarded_count: Vec<u32>,
}

impl Chess {
    /// Constructor.
    pub fn new(size: usize) -> Self {
        Self {
            trial_ops: 0,
            discard_ops: 0,
            queens: vec![(0..size).into_iter().collect(); size],
            discarded_pairs: Vec::new(),
            discarded_count: Vec::new()
        }
    }

    /// Solve queens problem.
    pub fn solve(&mut self) -> bool {
        let r = self.select_index();

        if r.is_none() {
            return true;
        }

        let index = r.unwrap();
        let values = self.select_values(index);
        let current_set = self.queens[index].clone();

        for value in values {
            if self.assign(index, value) {
                if self.solve() {
                    return true;
                }

                self.restore_last();
            }

            self.queens[index].clone_from(&current_set);
        }

        false
    }

    /// Get number of total tries of assignation.
    pub fn trials(&self) -> u64 {
        self.trial_ops
    }

    /// Get number of total discards.
    pub fn discards(&self) -> u64 {
        self.discard_ops
    }

    /// Assign a value to a row and propagate constraints.
    fn assign(&mut self, index: usize, value: usize) -> bool {
        self.trial_ops += 1;
        self.clear_row(index);
        self.discarded_count.push(0);

        if !self.propagate(index, value) {
            self.restore_last();
            return false;
        }

        self.push_candidate(index, value);
        true
    }

    /// Discard candidate values (constraints propagation).
    fn discard(&mut self, index: usize, value: usize) -> bool {
        if value >= self.queens.len() {
            return true;
        }

        if !self.remove_candidate(index, value) {
            return true;
        }

        self.discard_ops += 1;
        self.discarded_pairs.push((index, value));

        let count = self.discarded_count.pop().unwrap();
        self.discarded_count.push(count + 1);

        match self.queens[index].len() {
            0 => false,
            1 => self.propagate(index, self.get_value(index)),
            _ => true,
        }
    }

    /// Undo last assignation (restore constraints).
    fn restore_last(&mut self) {
        let count = self.discarded_count.pop().unwrap();

        for _ in 0..count {
            let (index, value) = self.discarded_pairs.pop().unwrap();
            self.push_candidate(index, value);
        }
    }

    /// Propagate constraints.
    fn propagate(&mut self, index: usize, value: usize) -> bool {
        for i in 0..self.queens.len() {
            if i != index {
                if !self.discard(i, value) || (value + index >= i && !self.discard(i, value + index - i)) || (value + i >= index && !self.discard(i, value + i - index)) {
                    return false;
                }
            }
        }

        true
    }

    /// Clear a row (empty candidates).
    fn clear_row(&mut self, index: usize) {
        self.queens[index].clear();
    }

    /// Push a candidate value back to a row.
    fn push_candidate(&mut self, index: usize, value: usize) {
        self.queens[index].insert(value);
    }

    /// Remove a candidate from a row.
    fn remove_candidate(&mut self, index: usize, value: usize) -> bool {
        self.queens[index].remove(&value)
    }

    /// Get index of an unsolved row (minimum remaining values heuristics).
    fn select_index(&self) -> Option<usize> {
        let mut min_size = self.queens.len() + 1;
        let mut min_index: usize = 0;

        for i in 0..self.queens.len() {
            let size = self.queens[i].len();

            if size > 1 && size < min_size {
                min_index = i;
                min_size = size;
            }
        }

        if min_size == self.queens.len() + 1 { None } else { Some(min_index) }
    }

    /// Select all available indices from a row.
    fn select_values(&self, index: usize) -> HashSet<usize> {
        self.queens[index].clone()
    }

    /// Get the __only__ value that is set in the array.
    fn get_value(&self, index: usize) -> usize {
        for &i in &self.queens[index] {
            return i;
        }

        panic!("Empty row")
    }
}

impl fmt::Display for Chess {
    /// Representation of the chessboard.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.queens.len() {
            if self.queens[i].len() == 1 {
                if let Err(x) = writeln!(f, "Queen {}: square {}", i + 1, self.get_value(i)) {
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
