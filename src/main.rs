use std::time::Instant;

use cadabra::*;
use Square::*;
use PieceType::*;
use MoveType::*;
//use Color::*;

fn main() {
    //let mut pos = Position::new_from_fen("4k3/8/2p5/1B6/8/8/8/8 b - - 0 1").unwrap();

    let mut pos = Position::new_from_start_pos();

    /*pos.make_move(Move::new_custom(e2 as u8, e3 as u8, Pawn, Quiet));
    pos.make_move(Move::new_custom(f7 as u8, f6 as u8, Pawn, Quiet));
    pos.make_move(Move::new_custom(f1 as u8, b5 as u8, Bishop, Quiet));
    */
    
    //let moves = pos.generate_moves();

    pos.pretty_print();

    //println!("{}", pos.generate_hv_pin_mask(White) | pos.generate_d12_pin_mask(White));

    let before = Instant::now();

    let mut res = PerftResult{ nodes: 0, captures: 0, promotions: 0, castles: 0, enpassants: 0 };

    let depth = 4;

    perft::<true>(&pos, depth, &mut res);

    println!(" Perft at depth {depth} done in {}ms\n{res}", before.elapsed().as_millis());
}
