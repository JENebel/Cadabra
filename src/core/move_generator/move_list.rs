use super::*;

pub type MoveList = heapless::Vec<Move, 128>;

pub trait MoveListFuncs {
    //fn pop_best(&mut self) -> Option<Move>;
    fn push_move(&mut self, moov: Move);
}

impl MoveListFuncs for MoveList {
    /*fn pop_best(&mut self) -> Option<Move> {
        let mut best_index = 0;

        for (i, m) in self.iter().enumerate() {
            let best_score = self[best_index].score;
            let score = m.score;

            if score > best_score {
                best_index = i
            }
        }

        let length = self.len();
        self.swap(length - 1, best_index);
        self.pop()
    }*/

    /// Unsafely inserts. This is not optimal, but increases performance by ~3%
    fn push_move(&mut self, moov: Move) {
        debug_assert!(self.len() + 1 < self.capacity());
        
        unsafe { self.push_unchecked(moov) }
    }
}