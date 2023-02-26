use super::*;

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
    pub fn evaluate(&self) -> i32 {
        let mut score: i32 = 0;

        for bb in 0..12 {
            let mut board = self.bitboards[bb];
            while let Some(square) = board.extract_bit() {
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

        if self.active_color == Color::White { score } else { -score } // Colud avoid branching here
    }
}
