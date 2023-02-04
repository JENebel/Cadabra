mod bitboard;
mod position;
mod attack_tables;
mod uci;
mod evaluation;
mod zobrist_constants;
mod move_generator;
mod definitions;
mod constants;
mod cmove;

use bitboard::*;
use position::*;
use attack_tables::*;
use uci::*;
use evaluation::*;
use zobrist_constants::*;
use move_generator::*;
use definitions::*;
use constants::*;
use cmove::*;

fn main() {
    //let mut pos = Position::new_from_fen("k7/8/8/8/3K4/8/8/8 w - - 0 1").unwrap();
    let mut pos = Position::new_from_start_pos();

    let moves = MoveList::new(&mut pos, false, false, None);

    println!("{}", moves.length());

    for m in moves {
        println!("{}", m)
    }
}