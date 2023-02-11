use std::collections::HashMap;

use cadabra::*;
use Square::*;
use Color::*;

fn main() {
    let pos = Position::new_from_fen("8/8/8/3K4/8/8/4R1b1/8 w - - 0 1").unwrap();

    let moves = pos.generate_moves_internal();

    //println!("{}", pos.generate_hv_pin_mask(White) | pos.generate_d12_pin_mask(White));

    println!("Moves: {}", moves.len());
    for m in moves {
        println!("{m}")
    }
}
