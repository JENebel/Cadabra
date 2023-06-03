use super::*;
use Color::*;
use PieceType::*;
use const_for::*;

const MATERIAL_WEIGHTS: [i16; 12] = [100, 320, 360, 520, 1000, 0, -100, -320, -360, -520, -1000, 0];

const STACKED_PAWN_PENALTY: i16 = 10;
const ISOLATED_PAWN_PENALTY: i16 = 10;
const PASSED_PAWN_BONUS: [i16; 8] = [ 0, 10, 30, 50, 75, 100, 150, 200 ];
// const SUPPORTED_PAWN_BONUS: i16 = 0;

const SEMI_OPEN_FILE_BONUS: i16 = 10;
const OPEN_FILE_BONUS: i16 = 15;

const PROTECTED_KING_BONUS: i16 = 5;

const KNIGHT_MOBILITY_BONUS: i16 = 1;
const BISHOP_MOBILITY_BONUS: i16 = 1;
const ROOK_MOBILITY_BONUS: i16 = 1;
const QUEEN_MOBILITY_BONUS: i16 = 1;


// PIECE SQUARE TABLES

/// Pawn positional score
pub const PAWN_SCORES: [i16; 64] = 
[
    90,  90,  90,  90,  90,  90,  90,  90,
    30,  30,  30,  40,  40,  30,  30,  30,
    20,  20,  20,  30,  30,  30,  20,  20,
    10,  10,  10,  20,  20,  10,  10,  10,
     5,   5,  10,  20,  20,   5,   5,   5,
     0,   0,   0,   5,   5,   0,   0,   0,
     0,   0,   0, -10, -10,   0,   0,   0,
     0,   0,   0,   0,   0,   0,   0,   0
];

/// Knight positional score
const KNIGHT_SCORES: [i16; 64] = 
[
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,  10,  10,   0,   0,  -5,
    -5,   5,  20,  20,  20,  20,   5,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,   5,  20,  10,  10,  20,   5,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5, -10,   0,   0,   0,   0, -10,  -5
];

/// Bishop positional score
const BISHOP_SCORES: [i16; 64] = 
[
     0,  0,   0,  0,  0,   0,  0, 0,
     0,  0,   0,  0,  0,   0,  0, 0,
     0,  0,   0, 10, 10,   0,  0, 0,
     0,  0,  10, 20, 20,  10,  0, 0,
     0,  0,  10, 20, 20,  10,  0, 0,
     0, 10,   0,  0,  0,   0, 10, 0,
     0, 30,   0,  0,  0,   0, 30, 0,
     0,  0, -10,  0,  0, -10,  0, 0
];

/// Rook positional score
const ROOK_SCORES: [i16; 64] = 
[
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  5,  10,  10,  5,  0,  0
];

// Queen positional score
const QUEEN_SCORES: [i16; 64] = 
[
   -20, -10, -10, -5, -5, -10, -10, -20,
   -10,   0,   0,  0,  0,   0,   0, -10,
   -10,   0,   5,  5,  5,   5,   0, -10,
    -5,   0,   5,  5,  5,   5,   0,  -5,
     0,   0,   5,  5,  5,   5,   0,  -5,
   -10,   5,   5,  5,  5,   5,   0, -10,
   -10,   0,   5,  0,  0,   0,   0, -10,
   -20, -10, -10, -5, -5, -10, -10, -20
];

/// King positional score
const KING_SCORES: [i16; 64] = 
[
     0, 0,  0,  0,   0,  0,  0, 0,
     0, 0,  5,  5,   5,  5,  0, 0,
     0, 5,  5, 10,  10,  5,  5, 0,
     0, 5, 10, 20,  20, 10,  5, 0,
     0, 5, 10, 20,  20, 10,  5, 0,
     0, 0,  5, 10,  10,  5,  0, 0,
     0, 5,  5, -5,  -5,  0,  5, 0,
     0, 0,  5,  0, -15,  0, 10, 0
];

/// Mirror positional score tables for opposite side
const MIRROR: [usize; 64] = 
[
	56, 57, 58, 59, 60, 61, 62, 63,
	48, 49, 50, 51, 52, 53, 54, 55,
	40, 41, 42, 43, 44, 45, 46, 47,
	32, 33, 34, 35, 36, 37, 38, 39,
	24, 25, 26, 27, 28, 29, 30, 31,
	16, 17, 18, 19, 20, 21, 22, 23,
	8,  9,  10, 11, 12, 13, 14, 15,
	0,  1,  2,  3,  4,  5,  6,  7
];

const ISOLATED_MASKS: [u64; 64] = generate_isolated_pawn_masks();
const WHITE_PASSED_PAWN_MASKS: [u64; 64] = generate_passed_pawn_masks(true);
const BLACK_PASSED_PAWN_MASKS: [u64; 64] = generate_passed_pawn_masks(false);

macro_rules! cond_val {
    ($cond: expr; $val: expr) => {
        if $cond { $val } else { 0 }
    };
}

impl Position {
    #[inline(always)]
    pub fn evaluate(&self) -> i16 {
        let mut score: i16 = 0;

        for bb in 0..12 {
            for square in self.bitboards[bb].map(|sq| sq as usize) {
                score += MATERIAL_WEIGHTS[bb];

                let (color, piece) = index_to_piece(bb);

                let piece_square_index = match color {
                    White => square,
                    Black => MIRROR[square]
                };

                let eval = match piece {
                    Pawn => {
                        // Piece square table
                        let piece_square_bonus = PAWN_SCORES[piece_square_index];

                        // Stacked pawn penalty
                        let stacked_pawns = (self.bb(color, Pawn) & FILE_MASKS[square]).pop_count() as i16;
                        let stack_penalty = (stacked_pawns - 1) * STACKED_PAWN_PENALTY;

                        // Isolated pawn penalty
                        let isolated_penalty = cond_val!((self.bb(color, Pawn) & ISOLATED_MASKS[square]).is_empty(); ISOLATED_PAWN_PENALTY);

                        // Supported pawn bonus
                        // let supported_pawns = (self.bb(color, Pawn) & pawn_attacks(square as u8, color)).pop_count() as i16;
                        // let supporting_pawn_bonus = supported_pawns * SUPPORTED_PAWN_BONUS;

                        // Passed pawn bonus
                        let passed_pawn_bonus = match color {
                            White => cond_val!((self.bb(Black, Pawn) & WHITE_PASSED_PAWN_MASKS[square]).is_empty(); PASSED_PAWN_BONUS[LOOKUP_RANK[square]]),
                            Black => cond_val!((self.bb(White, Pawn) & BLACK_PASSED_PAWN_MASKS[square]).is_empty(); PASSED_PAWN_BONUS[7 - LOOKUP_RANK[square]]),
                        };

                        piece_square_bonus - stack_penalty - isolated_penalty + passed_pawn_bonus// + supporting_pawn_bonus
                    },
                    Knight => {
                        // Piece square table
                        let piece_square_bonus = KNIGHT_SCORES[piece_square_index];

                        // Mobility bonus
                        let mobility_bonus = KNIGHT_MOBILITY_BONUS * (knight_attacks(square as u8) & !self.color_bb(color)).pop_count() as i16;

                        piece_square_bonus + mobility_bonus
                    },
                    Bishop => {
                        // Piece square table
                        let piece_square_bonus = BISHOP_SCORES[piece_square_index];

                        // Mobility bonus
                        let mobility_bonus = BISHOP_MOBILITY_BONUS * (d12_attacks(square as u8, self.all_occupancies) & !self.color_bb(color)).pop_count() as i16;
                        
                        piece_square_bonus + mobility_bonus
                    },
                    Rook => {
                        // Piece square table
                        let piece_square_bonus = ROOK_SCORES[piece_square_index];

                        // Semi open file bonus
                        let semi_open_bonus = cond_val!((self.bb(color, Pawn) & FILE_MASKS[square]).is_empty(); SEMI_OPEN_FILE_BONUS);

                        // Open file bonus
                        let open_bonus = cond_val!(((self.bb(White, Pawn) | self.bb(Black, Pawn)) & FILE_MASKS[square]).is_empty(); OPEN_FILE_BONUS);

                        // Mobility bonus
                        let mobility_bonus = ROOK_MOBILITY_BONUS * (hv_attacks(square as u8, self.all_occupancies) & !self.color_bb(color)).pop_count() as i16;
                        
                        piece_square_bonus + semi_open_bonus + open_bonus + mobility_bonus
                    },
                    Queen => {
                        // Piece square table
                        let piece_square_bonus = QUEEN_SCORES[piece_square_index];

                        // Mobility bonus
                        let mobility_bonus = QUEEN_MOBILITY_BONUS * ((d12_attacks(square as u8, self.all_occupancies) | hv_attacks(square as u8, self.all_occupancies)) & !self.color_bb(color)).pop_count() as i16;

                        piece_square_bonus + mobility_bonus
                    },
                    King => {
                        // Piece square table
                        let piece_square_bonus = KING_SCORES[piece_square_index];

                        // Semi open file penalty
                        let semi_open_penalty = cond_val!((self.bb(color, Pawn) & FILE_MASKS[square]).is_empty(); SEMI_OPEN_FILE_BONUS);

                        // Open file penalty
                        let open_penalty = cond_val!(((self.bb(White, Pawn) | self.bb(Black, Pawn)) & FILE_MASKS[square]).is_empty(); OPEN_FILE_BONUS);

                        // King safety bonus
                        let king_safety_bonus = (king_attacks(square as u8) & self.color_bb(color)).pop_count() as i16 * PROTECTED_KING_BONUS;

                        piece_square_bonus - semi_open_penalty - open_penalty + king_safety_bonus
                    },
                    Empty => unreachable!("Empty piece on board!"),
                };

                match color {
                    White => score += eval,
                    Black => score -= eval,
                }
            }
        }

        if self.active_color.is_white() { score } else { -score } // Colud avoid branching here
    }
}

const fn generate_isolated_pawn_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            if file > 0 {
                masks[rank*8+file] |= FILE_MASKS[file - 1];
            }

            if file < 7 {
                masks[rank*8+file] |= FILE_MASKS[file + 1];
            }
        })
    });

    masks
}

const fn generate_passed_pawn_masks(white: bool) -> [u64; 64] {
    let mut masks = [0; 64];

    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            let range = if white { 0..rank } else { rank+1..8 };
            const_for!(r in range => {
                masks[rank*8+file] |= 1 << (r*8+file);

                if file > 0 {
                    masks[rank*8+file] |= 1 << (r*8+file-1);
                }

                if file < 7 {
                    masks[rank*8+file] |= 1 << (r*8+file+1);
                }
            })
        })
    });

    masks
}