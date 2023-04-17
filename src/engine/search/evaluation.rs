use super::*;

pub const MATERIAL_WEIGHTS: [i16; 12] = [100, 300, 350, 500, 1000, 10000, -100, -300, -350, -500, -1000, -10000];
/*pub const STACKED_PAWN_PENALTY: i32 = -10;
pub const ISOLATED_PAWN_PENALTY: i32 = -10;
pub const PASSED_WHITE_PAWN_BONUS: [i32; 8] = [ 0, 10, 30, 50, 75, 100, 150, 200 ];
pub const PASSED_BLACK_PAWN_BONUS: [i32; 8] = [ 200, 150, 100, 75, 50, 30, 10, 0 ]; 
pub const SEMI_OPEN_FILE_SCORE: i32 = 10;
pub const OPEN_FILE_SCORE: i32 = 15;
pub const PROTECTED_KING_BONUS: i32 = 5;*/

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
pub const KNIGHT_SCORES: [i16; 64] = 
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
pub const BISHOP_SCORES: [i16; 64] = 
[
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   0,  10,  10,   0,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,  10,   0,   0,   0,   0,  10,   0,
     0,  30,   0,   0,   0,   0,  30,   0,
     0,   0, -10,   0,   0, -10,   0,   0

];

/// Rook positional score
pub const ROOK_SCORES: [i16; 64] = 
[
    50,  50,  50,  50,  50,  50,  50,  50,
    50,  50,  50,  50,  50,  50,  50,  50,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,   0,  20,  20,   0,   0,   0

];

/// King positional score
pub const KING_SCORES: [i16; 64] = 
[
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   5,   5,   5,   5,   0,   0,
     0,   5,   5,  10,  10,   5,   5,   0,
     0,   5,  10,  20,  20,  10,   5,   0,
     0,   5,  10,  20,  20,  10,   5,   0,
     0,   0,   5,  10,  10,   5,   0,   0,
     0,   5,   5,  -5,  -5,   0,   5,   0,
     0,   0,   5,   0, -15,   0,  10,   0
];

/// Mirror positional score tables for opposite side
pub const MIRRORED: [usize; 64] = 
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

impl Position {
    pub fn evaluate(&self) -> i16 {
        let mut score: i16 = 0;

        for bb in 0..12 {
            let board = self.bitboards[bb];
            for square in board {
                score += MATERIAL_WEIGHTS[bb];
                match bb {
                    // White pawns
                    0  => {
                        score += PAWN_SCORES[square as usize];
                    },
                    // White knight
                    1  => {
                        score += KNIGHT_SCORES[square as usize];
                    },
                    // White bishops
                    2  => {
                        score += BISHOP_SCORES[square as usize];
                    },
                    // White Rooks
                    3  => {
                        score += ROOK_SCORES[square as usize];
                    },
                    // White queen
                    4 => {
                    },
                    // White king
                    5  => {
                        score += KING_SCORES[square as usize];
                    },
                    // Black pawns
                    6  => {
                        score -= PAWN_SCORES[MIRRORED[square as usize]];
                    },
                    // Black knight
                    7  => {
                        score -= KNIGHT_SCORES[MIRRORED[square as usize]];
                    },
                    // Black bishop
                    8  => {
                        score -= BISHOP_SCORES[MIRRORED[square as usize]];
                    },
                    // Black rooks
                    9  => {
                        score -= ROOK_SCORES[MIRRORED[square as usize]];
                    },
                    // Black queen
                    10 => {
                    }
                    // Black king
                    11 => {
                        score -= KING_SCORES[MIRRORED[square as usize]];
                    },
                    _ => unreachable!()
                };
            }
        }

        if self.active_color.is_white() { score } else { -score } // Colud avoid branching here
    }
}
