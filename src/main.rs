use std::time::Instant;

use cadabra::*;
/*use Square::*;
use PieceType::*;
use MoveType::*;
use Color::*;*/

fn main() {
    let pos = Position::from_fen("3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap();

    pos.pretty_print();

    let depth = 5;

    let before = Instant::now();

    println!(" Found: {} moves at depth {depth} in {}ms", perft::<true>(&pos, depth), before.elapsed().as_millis());
}