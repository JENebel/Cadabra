use super::*;

#[derive(Copy, Clone)]
pub struct RepetitionTable {
    len: usize,
    hashes: [u64; 100]
}

impl RepetitionTable {
    pub fn new() -> Self {
        Self { len: 0, hashes: [0; 100] }
    }

    pub fn push(&mut self, hash: u64) {
        self.hashes[self.len] = hash;
        self.len += 1;
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
    
    pub fn is_in_3_fold_rep(&self, pos: &Position) -> bool {
        // It is first possible as the 8th halfmove since last irreversible move.
        // The check reduces performance impact
        if pos.half_moves >= 8 {
            // Count occurences of current position
            let hash = pos.zobrist_hash;
            let repetitions = self.hashes.iter().filter(|a| **a == hash).count();
            return repetitions >= 3
        }
        false
    }
}