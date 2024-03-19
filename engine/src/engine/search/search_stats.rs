use std::{ops::Add, iter::Sum};

use crate::engine::moove::Move;

// Result statistics of a search
pub struct SearchStats {
    pub nodes: u128,
    pub tt_hits: u128,
    pub time: u128, // millis
    pub best_move: Option<Move>,
}

impl Add<Self> for SearchStats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            nodes: self.nodes + rhs.nodes,
            tt_hits: self.tt_hits + rhs.tt_hits,
            time: self.time.max(rhs.time),
            best_move: None
        }
    }
}

impl Sum for SearchStats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter().reduce(|acc, res| acc + res).unwrap()
    }
}