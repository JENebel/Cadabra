use super::*;

include!(concat!(env!("OUT_DIR"), "/consts.rs"));

//Getters
pub fn get_pawn_attack_table(square: u8, color: Color) -> Bitboard {
    Bitboard::from_u64(
        if color == Color::White {
            WHITE_PAWN_ATTACKS[square as usize]
        }
        else {
            BLACK_PAWN_ATTACKS[square as usize]
        }
    )
}

pub fn get_knight_attack_table(square: u8) -> Bitboard {
    Bitboard::from_u64(
        KNIGHT_ATTACKS[square as usize]
    )
}

pub fn get_king_attack_table(square: u8) -> Bitboard {
    Bitboard::from_u64(
        KING_ATTACKS[square as usize]
    )
}

pub fn get_rook_attack_table(square: u8, occ: Bitboard) -> Bitboard {
    let attacks = SLIDING_ATTACKS[(ROOK_OFFSETS[square as usize] as u64 + pext(occ.to_u64(), ROOK_MASK[square as usize])) as usize];
    Bitboard::from_u64(
        attacks
    )
}
 
pub fn get_bishop_attack_table(square: u8, occ: Bitboard) -> Bitboard {
    let attacks = SLIDING_ATTACKS[(BISHOP_OFFSETS[square as usize] as u64 + pext(occ.to_u64(), BISHOP_MASK[square as usize])) as usize];
    Bitboard::from_u64(
        attacks
    )
}

pub fn get_queen_attack_table(square: u8, occ: Bitboard) -> Bitboard {
    let bishop = SLIDING_ATTACKS[(BISHOP_OFFSETS[square as usize] as u64 + pext(occ.to_u64(), BISHOP_MASK[square as usize])) as usize];
    let rook = SLIDING_ATTACKS[(ROOK_OFFSETS[square as usize] as u64 + pext(occ.to_u64(), ROOK_MASK[square as usize])) as usize];

    Bitboard::from_u64 (
        rook | bishop
    )
}

fn pext(bits: u64, mask: u64) -> u64 {
    unsafe { core::arch::x86_64::_pext_u64(bits, mask) }
}