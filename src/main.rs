use std::time::Instant;

use cadabra::*;
/*use Square::*;
use PieceType::*;
use MoveType::*;
use Color::*;*/

fn main() {
    let pos = Position::start_pos();

    //pos.pretty_print();

    let depth = 6;

    let before = Instant::now();

    println!(" Found: {} moves at depth {depth} in {}ms", perft::<false>(&pos, depth), before.elapsed().as_millis());
}