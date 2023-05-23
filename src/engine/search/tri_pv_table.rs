use std::fmt::Display;

use super::*;

#[derive(Clone)]
pub struct PVTable {
    pub pv_table: [[Option<Move>; MAX_PLY as usize]; MAX_PLY as usize],
    pub pv_lengths: [usize; MAX_PLY as usize],
}

impl PVTable {
    pub fn new() -> Self {
        Self {
            pv_table: [[None; MAX_PLY as usize]; MAX_PLY as usize],
            pv_lengths: [0; MAX_PLY as usize],
        }
    }

    pub fn best_move(&self) -> Option<Move> {
        self.pv_table[0][0]
    }

    pub fn insert_pv_node(&mut self, moove: Move, ply: u8) {
        let ply = ply as usize;

        self.pv_table[ply][ply] = Some(moove);

        for i in (ply + 1)..=self.pv_lengths[ply + 1] {
            self.pv_table[ply][i] = self.pv_table[ply + 1][i];
        }

        self.pv_lengths[ply] = self.pv_lengths[ply + 1];
    }
}

impl Display for PVTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.pv_lengths[0] {
            write!(f, "{} ", self.pv_table[0][i].unwrap())?;
        }
        Ok(())
    }
}