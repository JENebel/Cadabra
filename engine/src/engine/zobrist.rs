use const_for::const_for;

use super::{Color, PieceType, Position, piece::index_to_piece};

macro_rules! generate_keys {
    ($count: expr, $seed: expr) =>  {
        {
            let count: usize = $count;
            let mut seed: u32 = $seed;
            let mut keys = [0; $count];
            const_for!(i in 0..count => {
                let res = rand_u64(seed);
                seed = res.1;
                keys[i] = res.0;
            });

            keys
        }
    };
}

/// Is 0 for index 0 so xor does nothing when there is no ep
const ENPASSANT_KEYS: [u64; 64] = {
    let mut keys = generate_keys!(64, 0b10111100100101101000101001111111);
    keys[0] = 0;
    keys
};
const PIECE_KEYS: [u64; 12 * 64] = generate_keys!(12 * 64, 0b10111000110000001011100000111000);
const CASTLING_KEYS: [u64; 16] = generate_keys!(16, 0b01001001011100011110101101011001);
const SIDE_KEY: u64 = rand_u64(0b11011111101100101101100000101101).0;

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

        for i in 0..12 {
            let (piece_type, color) = index_to_piece(i);
            for sq in self.bitboards[i] {
                self.apply_piece_zobrist(piece_type, color, sq);
            }
        }

        self.apply_castling_zobrist();
        
        if self.active_color.is_black() {
            self.apply_side_zobrist()
        }

        if let Some(enpassant_square) = self.enpassant_sq() {
            self.apply_enpassant_zobrist(enpassant_square);
        }
    }
}

const fn rand_u32(state: u32) -> u32 {
    let mut num: u64 = state as u64;

    // XOR shift algorithm
    const_for!(_ in 0..27 => {
        num ^= num << 13;
        num ^= num >> 17;
        num ^= num << 5;
    });

    // return random number
    return num as u32;
}

// generate 64-bit pseudo random numbers
const fn rand_u64(state: u32) -> (u64, u32) {
    let n1 = rand_u32(state);
    let n2 = rand_u32(n1);
    let n3 = rand_u32(n2);
    
    // return random number
    return (n1 as u64 | ((n2 as u64) << 32), n3);
}