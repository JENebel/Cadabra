use super::*;
use bitintr::{Pext, Pdep};

#[inline(always)]
pub fn get_attacks(square: u8, color: Color, piece_type: PieceType, occupancies: Bitboard) -> u64 {
    match piece_type {
        PieceType::Pawn =>   pawn_attacks(square, color),
        PieceType::Knight => knight_attacks(square),
        PieceType::Bishop => d12_attacks(square, occupancies),
        PieceType::Rook =>   hv_attacks(square, occupancies),
        PieceType::Queen =>  d12_attacks(square, occupancies) | hv_attacks(square, occupancies),
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
pub fn hv_attacks(square: u8, occ: Bitboard) -> u64 {
    let offset = ROOK_OFFSETS[square as usize] as u64;
    let mask = ROOK_MASK[square as usize];
    let index = occ.as_u64().pext(mask);

    SLIDING_ATTACKS[(offset + index) as usize]
}

#[inline(always)]
pub fn d12_attacks(square: u8, occ: Bitboard) -> u64 {
    let offset = BISHOP_OFFSETS[square as usize] as u64;
    let mask = BISHOP_MASK[square as usize];
    let index = occ.as_u64().pext(mask);

    SLIDING_ATTACKS[(offset + index) as usize]
}

#[inline(always)]
pub fn pin_mask_h(occ: Bitboard, king_pos: u8, slider_pos: u8) -> u64 {
    let rank = RANK_MASKS[king_pos as usize];

    let pexed = occ.as_u64().pext(rank);

    let kp = LOOKUP_FILE[king_pos as usize];
    let sq = LOOKUP_FILE[slider_pos as usize];

    let index = 2048*kp + 256*sq + pexed as usize;
    let mask = PIN_MASKS[index];

    mask.pdep(rank)
}

#[inline(always)]
pub fn pin_mask_v(occ: Bitboard, king_pos: u8, slider_pos: u8) -> u64 {
    let file = FILE_MASKS[king_pos as usize];

    let pexed = occ.as_u64().pext(file);

    let kp = 7 - LOOKUP_RANK[king_pos as usize];    // Can maybe reverse bits somehow instead of 7 -   TODO
    let sq = 7 - LOOKUP_RANK[slider_pos as usize];
    let index = 2048*kp + 256*sq + pexed as usize;

    let mask = PIN_MASKS[index];

    mask.pdep(file)
}


#[inline(always)]
pub fn pin_mask_d1(occ: Bitboard, king_pos: u8, slider_pos: u8) -> u64 {
    let diagonal = D1_MASKS[king_pos as usize];

    let pexed = occ.as_u64().pext(diagonal);
    let kp = LOOKUP_D2[king_pos as usize];
    let sq = LOOKUP_D2[slider_pos as usize];

    let index = 2048*kp + 256*sq + pexed as usize;
    let mask = PIN_MASKS[index];

    mask.pdep(diagonal)
}

#[inline(always)]
pub fn pin_mask_d2(occ: Bitboard, king_pos: u8, slider_pos: u8) -> u64 {
    let diagonal = D2_MASKS[king_pos as usize];

    let pexed = occ.as_u64().pext(diagonal);

    // sq and kp are flipped to get correct mask
    let kp = LOOKUP_D1[king_pos as usize];
    let sq = LOOKUP_D1[slider_pos as usize];

    let index = 2048*kp + 256*sq + pexed as usize;
    let mask = PIN_MASKS[index];

    mask.pdep(diagonal)
}