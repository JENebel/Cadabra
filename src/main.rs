use std::collections::HashMap;

use cadabra::*;
use Square::*;
use Color::*;

fn main() {
    let pos = Position::new_from_fen("q5b1/1P1q4/3P4/rQ1K2Pb/2RPP3/5b2/r2b4/8 w - - 0 1").unwrap();

    //let moves = pos.generate_moves_internal();

    println!("{}", pos.generate_hv_pin_mask(White) | pos.generate_d12_pin_mask(White));

    /*println!("Moves: {}", moves.len());
    for m in moves {
        //println!("{m}")
    }*/
}