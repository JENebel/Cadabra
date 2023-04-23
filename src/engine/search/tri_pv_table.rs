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

    pub fn init_ply(&mut self, ply: u8) {
        self.pv_lengths[ply as usize] = ply as usize
    }

    pub fn best_move(&self) -> Option<Move> {
        self.pv_table[0][0]
    }

    pub fn insert_pv_node(&mut self, cmove: Move, ply: u8) {
        println!("Inserting PV node at ply {}", ply);
        let ply = ply as usize;

        self.pv_table[ply][ply] = Some(cmove);
        
        for next_ply in (ply + 1)..self.pv_lengths[ply + 1] {
            self.pv_table[ply][next_ply] = self.pv_table[ply + 1][next_ply];
        }

        self.pv_lengths[ply] = self.pv_lengths[ply + 1];
    }
}