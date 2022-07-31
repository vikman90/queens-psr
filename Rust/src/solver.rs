// N queens problem (Rust version)
// Copyleft 2022 Vikman - All rights revoked.
// July 31, 2022

use crate::chess::Chess;

pub struct Solver {
    /// Number of calls to assign().
    trial_ops: u64,
    /// Number of calls to discard().
    discard_ops: u64,
    /// Queens' positions (set of candidate positions).
    chess: Chess,
    /// Discarded candidates (index-value).
    discarded_pairs: Vec<Vec<(usize, usize)>>,
}

impl Solver {
    /// Constructor.
    pub fn new(chess: Chess) -> Self {
        Self {
            trial_ops: 0,
            discard_ops: 0,
            chess,
            discarded_pairs: Vec::new(),
        }
    }

    /// Solve queens problem.
    pub fn solve(&mut self) -> bool {
        let r = self.select_index();

        if r.is_none() {
            return true;
        }

        let index = r.unwrap();
        let values = self.chess[index].as_vec();
        let current_set = self.chess[index].clone();

        for value in values {
            if self.assign(index, value) {
                if self.solve() {
                    return true;
                }

                self.restore_last();
            }

            self.chess[index].clone_from(&current_set);
        }

        false
    }

    /// Get chess.
    pub fn chess(&self) -> &Chess {
        &self.chess
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
        self.chess[index].clear();
        self.discarded_pairs.push(Vec::new());

        if !self.propagate(index, value) {
            self.restore_last();
            return false;
        }

        self.chess[index].push(value);
        true
    }

    /// Discard candidate values (constraints propagation).
    fn discard(&mut self, index: usize, value: usize) -> bool {
        if value >= self.chess.len() {
            return true;
        }

        if !self.chess[index].remove(value) {
            return true;
        }

        self.discard_ops += 1;
        self.discarded_pairs.last_mut().unwrap().push((index,value));

        match self.chess[index].count() {
            0 => false,
            1 => self.propagate(index, self.chess[index].get_value()),
            _ => true,
        }
    }

    /// Undo last assignation (restore constraints).
    fn restore_last(&mut self) {
        let pairs = self.discarded_pairs.pop().unwrap();

        for (index, value) in pairs {
            self.chess[index].push(value);
        }
    }

    /// Propagate constraints.
    fn propagate(&mut self, index: usize, value: usize) -> bool {
        for i in 0..self.chess.len() {
            if i != index {
                if !self.discard(i, value) || (value + index >= i && !self.discard(i, value + index - i)) || (value + i >= index && !self.discard(i, value + i - index)) {
                    return false;
                }
            }
        }

        true
    }

    /// Get index of an unsolved row (minimum remaining values heuristics).
    pub fn select_index(&self) -> Option<usize> {
        let mut min_size = self.chess.len() + 1;
        let mut min_index: usize = 0;

        for i in 0..self.chess.len() {
            let size = self.chess[i].count();

            if size > 1 && size < min_size {
                min_index = i;
                min_size = size;
            }
        }

        if min_size == self.chess.len() + 1 { None } else { Some(min_index) }
    }
}
