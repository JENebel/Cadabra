use std::time::Instant;

use cadabra::*;

fn main() {
    let pos = Position::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq").unwrap();

    /*pos.make_move(Move::new_normal(Square::f3 as u8, Square::f6 as u8, PieceType::Queen, true));
    pos.make_move(Move::new_normal(Square::e8 as u8, Square::f8 as u8, PieceType::King, false));
    pos.make_move(Move::new_normal(Square::e5 as u8, Square::g6 as u8, PieceType::Knight, true));*/
    
    // let mut pos = Position::new_from_start_pos();

    /*println!("{}", ROOK_OFFSETS.iter().max().unwrap());
    println!("{}", BISHOP_OFFSETS.iter().max().unwrap());*/

    pos.pretty_print();

    let before = Instant::now();
    let mut result = PerftResult { nodes: 0, captures: 0, promotions: 0, castles: 0, enpassants: 0 };

    let depth = 1;

    perft::<true>(&pos, depth, &mut result);
    println!(" Perft at depth {depth} done in {}ms\n{result}", before.elapsed().as_millis());

    //println!("{}", Bitboard::from(END_RANKS_MASK));
}