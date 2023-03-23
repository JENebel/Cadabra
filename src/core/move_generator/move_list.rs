use super::*;
use stackvector::StackVec;

pub type MoveList = StackVec<[Move; 128]>;

pub trait LazySortedMoveList {
    fn pop_best(&mut self) -> Option<Move>;
}

impl LazySortedMoveList for MoveList {
    fn pop_best(&mut self) -> Option<Move> {
        let mut best_index = 0;

        for (i, m) in self.iter().enumerate() {
            let best_score = self[best_index].score;
            let score = m.score;

            if score > best_score {
                best_index = i
            }
        }

        let length = self.length;
        self.swap(length - 1, best_index);
        self.pop()
    }
}