use super::*;

/// Performs the PEXT CPU instruction
fn pext(bits: u64, mask: u64) -> u64 {
    unsafe { core::arch::x86_64::_pext_u64(bits, mask) }
}

#[inline(always)]
pub fn get_attacks(square: u8, color: Color, piece_type: PieceType, occupancies: Bitboard) -> u64 {
    match piece_type {
        PieceType::Pawn =>   pawn_attacks(square, color),
        PieceType::Knight => knight_attacks(square),
        PieceType::Bishop => bishop_attacks(square, occupancies),
        PieceType::Rook =>   rook_attacks(square, occupancies),
        PieceType::Queen =>  queen_attacks(square, occupancies),
        PieceType::King =>   king_attacks(square),
    }
}

/// Gets the possible pawn attacks from the current position
#[inline(always)]
pub fn pawn_attacks(square: u8, color: Color) -> u64 {
    if color == Color::White {
        WHITE_PAWN_ATTACKS[square as usize]
    }
    else {
        BLACK_PAWN_ATTACKS[square as usize]
    }
}

#[inline(always)]
pub fn knight_attacks(square: u8) -> u64 {
    KNIGHT_ATTACKS[square as usize]
}

#[inline(always)]
pub fn king_attacks(square: u8) -> u64 {
    KING_ATTACKS[square as usize]
}

#[inline(always)]
pub fn rook_attacks(square: u8, occ: Bitboard) -> u64 {
    let offset = ROOK_OFFSETS[square as usize] as u64;
    let mask = ROOK_MASK[square as usize];
    let index = pext(occ.as_u64(), mask);

    SLIDING_ATTACKS[(offset + index) as usize]
}

#[inline(always)]
pub fn bishop_attacks(square: u8, occ: Bitboard) -> u64 {
    let offset = BISHOP_OFFSETS[square as usize] as u64;
    let mask = BISHOP_MASK[square as usize];
    let index = pext(occ.as_u64(), mask);

    SLIDING_ATTACKS[(offset + index) as usize]
}

#[inline(always)]
pub fn queen_attacks(square: u8, occ: Bitboard) -> u64 {
    rook_attacks(square, occ) | bishop_attacks(square, occ)
}