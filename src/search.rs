use crate::{Position, Move};

const PVSIZE: usize = 128;

struct PVTable {
    pv_table: [[Option<Move>; PVSIZE]; PVSIZE],
    pv_lengths: [usize; PVSIZE],
}

impl PVTable {
    pub fn new() -> Self {
        Self {
            pv_table: [[None; PVSIZE]; PVSIZE],
            pv_lengths: [0; PVSIZE],
        }
    }

    pub fn best_move(&self) -> Option<Move> {
        self.pv_table[0][0]
    }

    pub fn insert_pv_node(&mut self, cmove: Move, ply: u8) {
        let ply = ply as usize;

        self.pv_table[ply][ply] = Some(cmove);
        
        for next_ply in (ply + 1)..self.pv_lengths[ply + 1] {
            self.pv_table[ply][next_ply] = self.pv_table[ply + 1][next_ply];
        }

        self.pv_lengths[ply] = self.pv_lengths[ply + 1];
    }
}

pub fn search(pos: &Position, depth: u8) {
    let mut pv_table = PVTable::new();

    let score = negamax(pos, -50000, 50000, depth, 0, &mut pv_table);

    println!("Estimated CP score: {score}");

    print!("bestmove {}\n", pv_table.best_move().unwrap());
}

fn negamax(pos: &Position, mut alpha: i32, beta: i32, depth: u8, ply: u8, pv_table: &mut PVTable) -> i32 {
    if depth == 0 {
        return pos.evaluate()
    };

    pv_table.pv_lengths[ply as usize] = ply as usize;

    let move_list = pos.generate_moves();

    for m in move_list {
        let mut new_pos = pos.clone();

        new_pos.make_move(m);

        let score = -negamax(&new_pos, -beta, -alpha, depth - 1, ply + 1, pv_table);

        if score > alpha {
            pv_table.insert_pv_node(m, ply);

            if score >= beta {
                return beta;
            }

            alpha = score;
        }
    }

    alpha
}