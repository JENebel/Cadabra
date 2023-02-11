use cadabra::*;
use Square::*;
use Color::*;

fn main() {
    let pos = Position::new_from_fen("6n1/4b3/2N5/6P1/2Kr1q2/6K1/p3R3/1QB2P2 w - - 0 1").unwrap();

    let king   = pos.king_position(White);
    let slider = f4
    as u8;

    println!("{}", Bitboard(pin_mask_hv(pos.all_occupancies, king, slider)));
    


    /*println!("{}", pin_mask_v (pos.all_occupancies, pos.king_position(White)));
    println!("{}", pin_mask_d1(pos.all_occupancies, pos.king_position(White)));
    println!("{}", pin_mask_d2(pos.all_occupancies, pos.king_position(White)));*/
}