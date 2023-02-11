use cadabra::*;
use Square::*;

fn main() {
    //let pos = Position::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq").unwrap();

    let king   = g7;
    let slider = c7;

    println!("{}", Bitboard(CHECK_PATH[king as usize * 64 + slider as usize]));
}