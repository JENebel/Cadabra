use std::{collections::HashMap};

use super::*;

pub fn perft<const DETAILED: bool>(pos: &Position, depth: u8) -> u64 {
    let moves = pos.generate_moves();

    if depth == 1 {
        return moves.len() as u64
    } else if depth == 0 {
        return 1
    }

    let mut nodes = 0;

    for m in moves {
        let mut copy = *pos;
        copy.make_move(m);

        let temp_res = perft::<false>(&copy, depth - 1);

        if DETAILED {
            println!("{}: {temp_res}", m.to_uci_string());
        }

        nodes += temp_res
    }

    nodes
}


pub fn debug_perft(pos: &Position, depth: u8) -> HashMap<String, u64> {
    assert!(depth > 0);

    let moves = pos.generate_moves();

    let mut result: HashMap<String, u64> = HashMap::new();

    for m in moves {
        let mut copy = *pos;
        copy.make_move(m);
        let sub_nodes = perft::<false>(&copy, depth - 1);
        result.insert(m.to_uci_string(), sub_nodes);
    }

    result
}