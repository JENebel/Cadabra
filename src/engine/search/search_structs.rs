use std::{ops::Add, iter::Sum};

use super::*;

/// The arguments provided in go command
#[derive(Copy, Clone)]
pub struct SearchMeta {
    pub max_depth: u8,
    pub time_target: u128,
    pub max_nodes: u128,
    pub ponder: bool,

}

const INF: u128 = 3155692597470; // 100 years in milliseconds, aka Infinite

impl SearchMeta {
    pub fn new_simple_depth(depth: u8) -> Self {
        Self::new(Some(depth), false, false, None, None, None, None, None).unwrap()
    }

    pub fn new(max_depth: Option<u8>, ponder: bool, infinite: bool, time_left: Option<u128>, inc: Option<u128>, movestogo: Option<u8>, nodes: Option<u128>, movetime: Option<u128>) -> Result<Self, String> {
        let time_target = if let Some(movetime) = movetime { // Fixed time search
            movetime
        } else if infinite { // Infinite search
            INF
        } else if let Some(time_left) = time_left { // Time control search
            let inc = inc.unwrap_or(0);
            let moves_to_go = movestogo.unwrap_or(30) as u128;

            if time_left < inc {
                inc - 500
            } else {
                (time_left + inc) / moves_to_go
            }
        } else {
            INF
        };

        Ok(Self {
            max_depth: max_depth.unwrap_or(MAX_PLY - 1),
            time_target,
            max_nodes: nodes.unwrap_or(u128::MAX),
            ponder,
        })
    }
}

#[derive(Clone)]
pub struct SearchContext {
    pub search: Search,
    pub search_meta: SearchMeta,
    pub pos: Position,
    pub pv_table: PVTable,

    pub nodes: u128,
    pub tt_hits : u128,
}

impl SearchContext {
    pub fn new(search: Search, search_meta: SearchMeta, pos: Position) -> Self {
        Self {
            search,
            search_meta,
            pos,
            pv_table: PVTable::new(),
            nodes: 0,
            tt_hits: 0,
        }
    }
}

pub struct SearchResult {
    pub nodes: u128,
    pub tt_hits: u128,
    pub time: u128, // millis
}

impl Add<Self> for SearchResult {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            nodes: self.nodes + rhs.nodes,
            tt_hits: self.tt_hits + rhs.tt_hits,
            time: self.time
        }
    }
}

impl Sum for SearchResult {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter().reduce(|acc, res| acc + res).unwrap()
    }
}