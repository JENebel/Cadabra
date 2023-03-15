use super::*;

pub struct MoveList {
    insert_index: usize,
    extract_index: usize,
    move_list: [Move; 128],
}

impl MoveList {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            insert_index: 0,
            extract_index: 0,
            move_list: [Default::default(); 128],
        }
    }

    /// Gets the amount of moves stored in the list
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.insert_index
    }

    /// Extracts a new move into the list
    #[inline(always)]
    pub(crate) fn insert(&mut self, new_move: Move) {
        self.move_list[self.insert_index] = new_move;
        self.insert_index += 1;
    }

    /// Extracts the best move in the list
    #[inline(always)]
    pub fn next_best(&mut self) -> Option<Move> {
        if self.extract_index == self.insert_index {
            return None
        };

        let mut best_index = self.extract_index;

        for i in self.extract_index..self.insert_index {
            let best_score = self.move_list[best_index].score;
            let score = self.move_list[i].score;

            if score > best_score {
                best_index = i
            }
        }

        self.move_list.swap(self.extract_index, best_index);

        let extracted = self.move_list[self.extract_index];
        self.extract_index += 1;

        Some(extracted)
    }
}

impl Iterator for MoveList {
    type Item = Move;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.extract_index == self.insert_index {
            return None
        };

        let extracted = self.move_list[self.extract_index];
        self.extract_index += 1;
        Some(extracted)
    }
}