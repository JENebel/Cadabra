include!(concat!(env!("OUT_DIR"), "/sliding_attacks.rs"));
use super::*;
use bitintr::Pext;

pub const WHITE_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(true);
pub const BLACK_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(false);
pub const KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();
pub const KING_ATTACKS: [u64; 64] = generate_king_attacks();

#[test]
fn brian() {
    println!("{}", Bitboard(HV_ATTACK_TABLE_MASKS[12]))
}

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
    if color.is_white() {
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
    let offset = HV_ATTACK_OFFSETS[square as usize] as u64;
    let mask = HV_ATTACK_TABLE_MASKS[square as usize];
    let index = occ.as_u64().pext(mask);

    HV_SLIDING_ATTACKS[(offset + index) as usize]
}

#[inline(always)]
pub fn d12_attacks(square: u8, occ: Bitboard) -> u64 {
    let offset = D12_ATTACK_OFFSETS[square as usize] as u64;
    let mask = D12_ATTACK_TABLE_MASKS[square as usize];
    let index = occ.as_u64().pext(mask);

    D12_SLIDING_ATTACKS[(offset + index) as usize]
}

const fn generate_pawn_attacks(color: bool) -> [u64; 64] {
    let mut attacks = [0; 64];
    
    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if color {
                if file != 7 { result = result | base >> 7 as u64 }
                if file != 0 { result = result | base >> 9 as u64 }
                
            } else {
                if file != 0 { result = result | base << 7 as u64 }
                if file != 7 { result = result | base << 9 as u64 }
            }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_knight_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if rank > 1 && file < 7 { result = result | base >> 15 as u64 }
            if rank > 0 && file < 6 { result = result | base >> 6 as u64 }

            if rank < 7 && file < 6 { result = result | base << 10 as u64 }
            if rank < 6 && file < 7 { result = result | base << 17 as u64 }

            if rank > 1 && file > 0 { result = result | base >> 17 as u64 }
            if rank > 0 && file > 1 { result = result | base >> 10 as u64 }

            if rank < 7 && file > 1 { result = result | base << 6 as u64 }
            if rank < 6 && file > 0 { result = result | base << 15 as u64 }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_king_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if rank > 0 { result = result | base >> 8 as u64 }
            if file > 0 { result = result | base >> 1 as u64 }
            if rank < 7 { result = result | base << 8 as u64 }
            if file < 7 { result = result | base << 1 as u64 }

            if file > 0 && rank > 0 { result = result | base >> 9 as u64 }
            if file < 7 && rank > 0 { result = result | base >> 7 as u64 }
            if file > 0 && rank < 7 { result = result | base << 7 as u64 }
            if file < 7 && rank < 7 { result = result | base << 9 as u64 }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}