include!(concat!(env!("OUT_DIR"), "/sliding_attacks.rs"));
use super::*;
use bitintr::Pext;
use const_for::*;
use PieceType::*;

pub const WHITE_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(true);
pub const BLACK_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(false);
pub const KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();
pub const KING_ATTACKS: [u64; 64] = generate_king_attacks();

pub fn get_attacks(square: u8, color: Color, piece_type: PieceType, occupancies: Bitboard) -> u64 {
    match piece_type {
        Pawn =>   pawn_attacks(square, color),
        Knight => knight_attacks(square),
        Bishop => d12_attacks(square, occupancies),
        Rook =>   hv_attacks(square, occupancies),
        Queen =>  d12_attacks(square, occupancies) | hv_attacks(square, occupancies),
        King =>   king_attacks(square),
        Empty =>  unreachable!("Cannot get attacks for empty piece!")
    }
}

/// Gets the possible pawn attacks from the current position
pub fn pawn_attacks(square: u8, color: Color) -> u64 {
    if color.is_white() {
        WHITE_PAWN_ATTACKS[square as usize]
    }
    else {
        BLACK_PAWN_ATTACKS[square as usize]
    }
}

pub fn knight_attacks(square: u8) -> u64 {
    KNIGHT_ATTACKS[square as usize]
}

pub fn king_attacks(square: u8) -> u64 {
    KING_ATTACKS[square as usize]
}

pub fn hv_attacks(square: u8, occ: Bitboard) -> u64 {
    let offset = HV_ATTACK_OFFSETS[square as usize] as u64;
    let mask = HV_ATTACK_TABLE_MASKS[square as usize];
    let index = occ.as_u64().pext(mask);

    HV_SLIDING_ATTACKS[(offset + index) as usize]
}

pub fn d12_attacks(square: u8, occ: Bitboard) -> u64 {
    let offset = D12_ATTACK_OFFSETS[square as usize] as u64;
    let mask = D12_ATTACK_TABLE_MASKS[square as usize];
    let index = occ.as_u64().pext(mask);

    D12_SLIDING_ATTACKS[(offset + index) as usize]
}

const fn generate_pawn_attacks(color: bool) -> [u64; 64] {
    let mut attacks = [0; 64];
    
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            let index = rank*8+file;
            let src: u64 = 1 << index;
            
            if color {
                if file != 7 { attacks[index] |= src >> 7_u64 }
                if file != 0 { attacks[index] |= src >> 9_u64 }
                
            } else {
                if file != 0 { attacks[index] |= src << 7_u64 }
                if file != 7 { attacks[index] |= src << 9_u64 }
            }
        });
    });

    attacks
}

const fn generate_knight_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            let index = rank*8+file;
            let src: u64 = 1 << index;
            
            if rank > 1 && file < 7 { attacks[index] |= src >> 15_u64 }
            if rank > 0 && file < 6 { attacks[index] |= src >> 6_u64 }

            if rank < 7 && file < 6 { attacks[index] |= src << 10_u64 }
            if rank < 6 && file < 7 { attacks[index] |= src << 17_u64 }

            if rank > 1 && file > 0 { attacks[index] |= src >> 17_u64 }
            if rank > 0 && file > 1 { attacks[index] |= src >> 10_u64 }
            
            if rank < 6 && file > 0 { attacks[index] |= src << 15_u64 }
            if rank < 7 && file > 1 { attacks[index] |= src << 6_u64 }
        });
    });
    
    attacks
}

const fn generate_king_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            let index = rank*8+file;
            let src: u64 = 1 << index;
            
            if rank > 0 { attacks[index] |= src >> 8_u64 }
            if file > 0 { attacks[index] |= src >> 1_u64 }
            if rank < 7 { attacks[index] |= src << 8_u64 }
            if file < 7 { attacks[index] |= src << 1_u64 }

            if file > 0 && rank > 0 { attacks[index] |= src >> 9_u64 }
            if file < 7 && rank > 0 { attacks[index] |= src >> 7_u64 }
            if file > 0 && rank < 7 { attacks[index] |= src << 7_u64 }
            if file < 7 && rank < 7 { attacks[index] |= src << 9_u64 }
        });
    });

    attacks
}