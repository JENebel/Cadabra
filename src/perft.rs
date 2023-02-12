use std::fmt::Display;

use super::*;

pub struct PerftResult {
    pub nodes: u64,
    pub captures: u64,
    pub promotions: u64,
    pub castles: u64,
    pub enpassants: u64,
}

impl Display for PerftResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " Moves: {}\n Captures: {}\n Promotions: {}\n Castles: {}, \n E.P.: {}", self.nodes, self.captures, self.promotions, self.castles, self.enpassants)
    }
}

pub fn perft<const DETAILED: bool>(pos: &Position, depth: u8, result: &mut PerftResult) {
    if depth == 0 {
        return;
    }

    let moves = pos.generate_moves();

    for m in moves {
        if depth == 1 {
            result.nodes += 1;
            if m.is_capture() {
                result.captures += 1
            }
            if m.is_promotion() {
                result.promotions += 1
            }
            if m.is_castling() {
                result.castles += 1
            }
            if m.move_type == MoveType::EnpassantCapture {
                result.enpassants += 1;
                result.captures += 1
            }
        }

        let mut copy = *pos;
        copy.make_move(m);
        let sub_res = result.nodes;
        perft::<false>(&copy, depth - 1, result);
        if DETAILED {
            println!("{}:\t{m}", result.nodes - sub_res)
        }
    }
}