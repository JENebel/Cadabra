use super::*;

#[derive(Clone)]
pub struct RepetitionTable {
    hashes: heapless::Vec<u64, 100>
}

impl RepetitionTable {
    pub fn new() -> Self {
        Self { hashes: heapless::Vec::new() }
    }

    pub fn push(&mut self, hash: u64) {
        unsafe { self.hashes.push_unchecked(hash) }
    }

    pub fn clear(&mut self) {
        self.hashes.clear()
    }
    
    pub fn is_in_3_fold_rep(&self, pos: &Position) -> bool {
        // It is first possible as the 8th halfmove since last irreversible move.
        // The check reduces performance impact
        if pos.half_moves >= 8 {
            // Count occurences of current position
            let repetitins = self.hashes.iter().filter(|a| *a == &pos.zobrist_hash).count();
            return repetitins >= 3
        }
        false
    }
    
}