use std::mem;

use super::{Color, PieceType, Position};
use const_random::*;

/// Is 0 for index 0 so xor does nothing when there is no ep
const ENPASSANT_KEYS: [u64; 64] = unsafe { mem::transmute(const_random!([u8; 512])) };
const PIECE_KEYS: [u64; 12 * 64] = unsafe { mem::transmute(const_random!([u8; 6144])) };
const CASTLING_KEYS: [u64; 16] = unsafe { mem::transmute(const_random!([u8; 128])) };
const SIDE_KEY: u64 = const_random!(u64);

#[derive(Copy, Clone, PartialEq)]
pub struct Zobrist {
    pub hash: u64
}

impl Position {
    pub fn apply_side_zobrist(&mut self) {
        self.zobrist_hash ^= SIDE_KEY
    }

    pub fn apply_piece_zobrist(&mut self, color: Color, piece_type: PieceType, square: u8) {
        self.zobrist_hash ^= PIECE_KEYS[piece_type.index(color) * 12 + square as usize]
    }
    
    pub fn apply_enpassant_zobrist(&mut self, square: u8) {
        self.zobrist_hash ^= ENPASSANT_KEYS[square as usize]
    }

    pub fn apply_castling_zobrist(&mut self) {
        self.zobrist_hash ^= CASTLING_KEYS[self.castling_ability.byte as usize];
    }

    /// Creates a zobrist hash from scratch for the current position
    pub fn generate_zobrist_hash(&mut self) {
        self.zobrist_hash = 0;

        for piece in 0..12 {
            let bb = self.bitboards[piece];
            for square in bb {
                self.zobrist_hash ^= PIECE_KEYS[piece * 12 + square as usize];
            }
        }

        self.apply_castling_zobrist();
        
        if self.active_color.is_black() {
            self.apply_side_zobrist()
        }

        if !self.enpassant_square.is_empty() {
            self.apply_enpassant_zobrist(self.enpassant_square.least_significant());
        }
    }
}