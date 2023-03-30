use std::sync::{mpsc::Receiver, RwLock, Arc};
use super::*;

/// The arguments provided in go command
pub struct SearchInfo {
    depth: u8,
}

impl SearchInfo {
    pub fn new(depth: u8) -> Self {
        Self { depth }
    }
}

/// Messages sent between main and search thread
pub enum SearchMessage {
    Done(Move),
    Stop,
    PonderHit,
}

#[allow(dead_code)]
pub struct SearchContext {
    search_info: SearchInfo, 
    pos: Position, 
    settings: Settings, 
    receiver: Receiver<SearchMessage>,
    is_stopping: Arc<RwLock<bool>>,
}

impl SearchContext {
    pub fn new(search_info: SearchInfo, pos: Position, settings: Settings, receiver: Receiver<SearchMessage>) -> Self {
        Self {
            search_info,
            pos,
            settings,
            receiver,
            is_stopping: Arc::new(RwLock::new(false))
        }
    }
}

pub fn search(context: SearchContext) {
    let mut pv_table = PVTable::new();

    let info = context.search_info;

    let score = negamax(&context.pos, -50000, 50000, info.depth, 0, &mut pv_table);

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