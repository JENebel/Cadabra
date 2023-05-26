use std::time::Instant;

use super::*;

#[derive(Clone)]
pub struct SearchContext {
    pub search: Search,
    pub search_meta: SearchArgs,
    pub pos: Position,
    pub pv_table: PVTable,
    pub killer_moves: [[Option<Move>; MAX_PLY as usize]; KILLER_MOVE_COUNT],
    pub history_moves: [[i16; 64]; 12],
    pub start_time: Instant,
    pub is_printing: bool,

    pub nodes: u128,
    pub qui_nodes: u128,
    pub tt_hits : u128,
}

impl SearchContext {
    pub fn new(search: Search, search_meta: SearchArgs, pos: Position, start_time: Instant, is_printing: bool) -> Self {
        Self {
            search,
            search_meta,
            pos,
            pv_table: PVTable::new(),
            killer_moves: [[None; MAX_PLY as usize]; KILLER_MOVE_COUNT],
            history_moves: [[0; 64]; 12],
            start_time,
            is_printing,
            nodes: 0,
            qui_nodes: 0,
            tt_hits: 0,
        }
    }

    /// Returns true if the search should continue, as well as mark as stopping if the time is up
    pub fn exceeded_time_target(&self) -> bool {
        self.start_time.elapsed().as_millis() > self.search_meta.time_target
    }

    pub fn insert_killer_move(&mut self, moove: Move, ply: u8) {
        for i in (1..KILLER_MOVE_COUNT).rev() {
            self.killer_moves[i][ply as usize] = self.killer_moves[i - 1][ply as usize];
        }

        self.killer_moves[0][ply as usize] = Some(moove);
    }
}