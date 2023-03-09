use crate::{Color, PieceType, Position};

const PIECE_KEYS: [u64; 12 * 64] = generate_piece_keys(); // SHOULD BE FIXED TO NOT USE PIECE_TYPE! TODO
const ENPASSANT_KEYS: [u64; 64] = generate_enpassant_keys();
const CASTLING_KEYS: [u64; 16] = generate_castle_keys();
const SIDE_KEY: u64 = get_random_u64_number(4084590338).0;

impl Position {
    #[inline(always)]
    pub fn apply_side_zobrist(&mut self) {
        self.zobrist_hash ^= SIDE_KEY
    }

    #[inline(always)]
    pub fn apply_piece_zobrist(&mut self, color: Color, piece_type: PieceType, square: u8) {
        self.zobrist_hash ^= PIECE_KEYS[piece_type.index(color) * 12 + square as usize]
    }

    #[inline(always)]
    pub fn apply_enpassant_zobrist(&mut self, square: u8) {
        self.zobrist_hash ^= ENPASSANT_KEYS[square as usize]
    }

    #[inline(always)]
    pub fn apply_castling_zobrist(&mut self) {
        self.zobrist_hash ^= CASTLING_KEYS[self.castling_ability as usize];
    }

    /// Creates a zobrist hash from scratch for the current position
    pub fn generate_zobrist_hash(&mut self) {
        self.zobrist_hash = 0;

        for piece in 0..12 {
            let mut bb = self.bitboards[piece];
            while let Some(square) = bb.extract_bit() {
                self.zobrist_hash ^= PIECE_KEYS[piece * 12 + square as usize];
            }
        }

        self.apply_castling_zobrist();
        
        if self.active_color == Color::Black {
            self.apply_side_zobrist()
        }

        if self.enpassant_square.is_not_empty() {
            self.apply_enpassant_zobrist(self.enpassant_square.least_significant());
        }
    }
}

macro_rules! const_for {
    ($var: ident, $from: expr => $to: expr; $body: expr) => {
        {
            let mut $var = 0;
            while $var < $to {
                $body

                $var += 1;
            }
        }
    };
}

pub const fn generate_castle_keys() -> [u64; 16] {
    let mut keys = [0; 16];

    let mut state = 3667794840;

    const_for!(i, 0 => 16; {
        let res = get_random_u64_number(state);
        state = res.1;
        keys[i] = res.0;
    });

    keys
}

pub const fn generate_enpassant_keys() -> [u64; 64] {
    let mut keys = [0; 64];

    let mut sq = 0;
    let mut state = 862131765;

    while sq < 64 {
        let res = get_random_u64_number(state);
        state = res.1;
        keys[sq] = res.0;
        sq+=1;
    }

    keys
}

pub const fn generate_piece_keys() -> [u64; 12 * 64] {
    let mut keys = [0; 12 * 64];

    let mut p  = 0;
    let mut sq;
    let mut state = 2828886037;

    while p < 12 {
        sq = 0;
        while sq < 64 {
            let res = get_random_u64_number(state);
            state = res.1;
            keys[p * 12 + sq] = res.0;
            sq+=1;
        }
        p+=1
    }

    keys
}

pub const fn get_random_u32_number(state: u32) -> u32{
    let mut num: u64 = state as u64;

    // XOR shift algorithm
    num ^= num << 13;
    num ^= num >> 17;
    num ^= num << 5;

    // return random number
    return num as u32;
}

// generate 64-bit pseudo legal numbers
pub const fn get_random_u64_number(state: u32) -> (u64, u32) {
    // define 4 random numbers
    let n1 = get_random_u32_number(state);
    let n2 = get_random_u32_number(n1);
    let n3 = get_random_u32_number(n2);
    let n4 = get_random_u32_number(n3);
    
    // return random number
    return (n1 as u64 | ((n2 as u64) << 16) | ((n3 as u64) << 32) | ((n4 as u64) << 48), n4);
}