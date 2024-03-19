use std::ops::{Add, Sub};

use super::*;
use Color::*;
use PieceType::*;
use const_for::*;

pub type WeightArray = [i16; WEIGHT_COUNT];

pub const WEIGHTS: WeightArray = EVAL_CONSTS;
pub const WEIGHT_COUNT: usize = 798;
//pub const WEIGHTS: [i16; 798] = [332, 348, 533, 1005, 330, 334, 508, 971, 18, 2, 7, -7, 6, -4, -5, -2, 343, 100, 162, 9, -34, 6, 199, -7, 57, 74, 66, -7, 64, 25, 90, 40, 15, 33, -10, 18, 2, 68, 16, -8, -49, 24, -33, -11, -6, 15, 35, -44, -29, -22, 0, 11, 22, 28, 20, 23, -37, 22, -51, -42, 3, 32, 36, -14, 78, 21, -1, 11, -8, 9, 3, 5, 6, -6, 0, 4, 4, 1, 5, -6, -25, 107, 36, 44, 99, 63, 17, 96, 65, 37, 66, 9, 6, 7, 19, 36, 35, 38, 19, 3, -5, -7, -6, 16, 46, 2, 8, -23, -16, -6, -16, 19, 22, 30, -21, -14, 13, -6, -6, 1, 32, 14, 26, 5, 28, 1, -8, 7, 79, 41, 24, 16, -3, 4, 10, -4, -78, -65, 1, -34, -19, -36, -47, -87, -79, 29, 76, 76, 51, 48, 12, -40, -15, 98, 82, 51, 106, 83, 47, -22, -17, 4, 48, 83, 54, 21, 3, 48, -1, -10, 11, 0, 53, 57, 52, 16, -15, 37, 10, 22, 48, 8, 6, 63, -82, -103, -37, -9, -32, 35, -77, -1, -14, 12, -45, -22, -58, 35, -19, -225, 33, -43, 18, -32, -51, -22, -83, -68, -11, 40, -2, 12, -28, -14, -46, -81, -28, -4, 11, 13, -1, -28, -21, -46, -58, 27, 10, 24, 24, -21, 2, -52, -12, -8, 6, 19, 15, 16, -19, -28, -42, -56, 0, -3, -2, -26, -27, -88, -54, -23, -30, -29, 19, -2, 12, -45, -135, -20, -62, -42, -24, -69, -55, -11, 61, 74, -35, 12, -63, -12, -10, 21, -40, -4, -31, 27, -22, 78, 15, -3, 31, 11, 9, 21, -10, 57, 112, 47, 12, 38, 18, 32, 13, 55, -11, -26, -12, 20, -16, 6, 50, 15, 29, -21, -3, -20, 25, 8, 10, 13, 26, -1, 99, -23, 120, 24, -6, 30, 38, -31, -64, -41, -4, -15, 8, -37, -110, -149, 1, -17, -7, -2, -10, -13, -23, -21, 14, 20, 16, -17, 18, -11, 17, -67, -22, 12, -6, 9, 24, 13, 0, 5, 10, 40, 14, 24, 23, -20, 36, -5, -11, 28, -11, 39, -10, 18, -22, -27, 0, 8, 15, -4, 21, 3, -37, 13, -36, -35, -54, 17, -12, -20, 0, -55, -43, -27, -26, -10, -13, -55, -27, -10, 28, 83, 44, 61, 25, 58, 119, 98, 51, 23, 63, 120, 129, 96, 123, 68, -51, 44, 56, 41, 61, 58, 105, 87, -6, -26, 14, 48, 23, 2, 20, 21, 9, -121, -32, 13, 22, 5, 56, -13, -26, -53, -24, -22, -23, -31, -5, -9, 16, -12, 16, -12, 3, 21, -48, -5, -9, 3, 6, 11, -4, -11, -71, -21, 6, 0, 16, 12, 16, 31, 22, -1, 26, 22, 17, -13, -19, 7, -8, -1, 43, 12, 11, 8, -7, 6, -4, 6, 24, 18, 20, 5, -6, 19, 15, 14, 4, 44, 22, 2, -10, 9, -11, 3, 11, 9, 9, 7, -14, -10, -15, -24, -21, -27, -22, 5, -21, -20, -29, -30, 6, -24, -16, 2, -19, -27, -18, -44, 6, -7, 57, 11, 86, 102, 182, 39, 22, -22, 5, -4, 10, 33, -90, 63, -10, -26, 16, 9, 19, 32, 65, 52, 12, -19, 5, -17, 67, 35, -9, -39, 3, 0, -1, -41, -11, -7, -6, -26, 18, -20, -7, 4, -7, 21, -2, 2, 6, -13, -10, 47, 32, 15, -22, -43, 28, -4, -38, -4, 31, -54, 3, -127, 3, 40, 25, 43, -9, 6, -66, -1, 27, 65, 61, 34, 32, 18, 35, 10, 17, 36, 20, 17, 49, 2, 16, 18, -4, 41, 48, 41, 13, 46, 51, 4, -8, 47, 39, 43, 16, 54, 43, 25, 26, 4, 28, 16, 18, -13, -1, -24, 22, -18, -14, -60, -29, -28, -41, -96, -14, 12, -31, -58, -54, -44, 8, -56, -35, 117, 67, 201, 112, 225, 167, -9, 112, 147, 11, 0, 49, -127, 11, -72, 50, 68, -27, -57, -71, -150, -251, -91, 24, 25, -17, -103, -84, -86, -27, -58, 59, -73, -24, -12, -38, 19, -45, -57, -39, 28, 16, -33, 41, 2, -30, -51, 105, 16, 34, -108, -5, 0, -10, 21, -129, 53, 36, -95, -4, -53, 3, 29, -72, 15, 14, 42, 21, -2, 39, -51, 25, 11, 49, 38, 43, 39, 61, 20, 16, 34, 47, 62, 50, 36, 22, 33, 18, 23, 41, 48, 43, 32, 35, 15, -23, 58, 24, 24, 22, 21, 22, 6, 6, -11, 2, 15, 16, 10, 13, -2, -33, 9, 1, 26, 18, -5, -10, -18, 62, -48, 0, -2, -46, -21, -40, -59, 9, 13, 15, 15, 34, 56, 76, 120, 233, 228, 6, 7, 3, -3, 4, -2, 1, 3, -4, -1, 2, -2];

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

const PAWN_PHASE: i16 = 0;
const KNIGHT_PHASE: i16 = 1;
const BISHOP_PHASE: i16 = 1;
const ROOK_PHASE: i16 = 2;
const QUEEN_PHASE: i16 = 4;
const TOTAL_PHASE: i16 = PAWN_PHASE * 16 + KNIGHT_PHASE * 4 + BISHOP_PHASE * 4 + ROOK_PHASE * 4 + QUEEN_PHASE * 2;

const ISOLATED_MASKS: [u64; 64] = generate_isolated_pawn_masks();
const WHITE_PASSED_PAWN_MASKS: [u64; 64] = generate_passed_pawn_masks(true);
const BLACK_PASSED_PAWN_MASKS: [u64; 64] = generate_passed_pawn_masks(false);

macro_rules! cond_val {
    ($cond: expr; $val: expr) => {
        if $cond { $val } else { 0 }
    };
}

struct ScorePair(i16, i16);

impl ScorePair {
    const ZERO: ScorePair = ScorePair(0, 0);

    fn early(&self) -> i16 { self.0 }
    fn late(&self) -> i16 { self.1 }
}

impl From<(i16, i16)> for ScorePair {
    fn from((early, late): (i16, i16)) -> ScorePair {
        ScorePair(early, late)
    }
}

impl Add<ScorePair> for ScorePair {
    type Output = ScorePair;

    fn add(self, rhs: ScorePair) -> ScorePair {
        ScorePair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<ScorePair> for ScorePair {
    type Output = ScorePair;

    fn sub(self, rhs: ScorePair) -> ScorePair {
        ScorePair(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<i16> for ScorePair {
    type Output = ScorePair;

    fn add(self, rhs: i16) -> ScorePair {
        ScorePair(self.0 + rhs, self.1 + rhs)
    }
}

impl Sub<i16> for ScorePair {
    type Output = ScorePair;

    fn sub(self, rhs: i16) -> ScorePair {
        ScorePair(self.0 - rhs, self.1 - rhs)
    }
}

impl Position {
    pub fn count_pieces(&self, piece: PieceType) -> i16 {
        (self.bb(White, piece).pop_count() + self.bb(Black, piece).pop_count()) as i16
    }

    #[inline(always)]
    pub fn evaluate(&self, evaluator: impl Evaluator) -> i16 {
        let mut score: ScorePair = ScorePair::ZERO;

        for bb in 0..12 {
            for square in self.bitboards[bb].map(|sq| sq as usize) {
                score = score + ScorePair::from((evaluator.material_weights_early()[bb], evaluator.material_weights_late()[bb]));

                let (color, piece) = index_to_piece(bb);

                let piece_square_index = match color {
                    White => square,
                    Black => MIRROR[square]
                };

                let eval = ScorePair::from(match piece {
                    Pawn => {
                        // Stacked pawn penalty
                        let stacked_pawns = (self.bb(color, Pawn) & FILE_MASKS[square]).pop_count() as i16;
                        let stack_penalty = (stacked_pawns - 1) * evaluator.stacked_pawn_penalty();

                        // Isolated pawn penalty
                        let isolated_penalty = cond_val!((self.bb(color, Pawn) & ISOLATED_MASKS[square]).is_empty(); evaluator.isolated_pawn_penalty());

                        // Supported pawn bonus
                        // let supported_pawns = (self.bb(color, Pawn) & pawn_attacks(square as u8, color)).pop_count() as i16;
                        // let supporting_pawn_bonus = supported_pawns * SUPPORTED_PAWN_BONUS;

                        // Passed pawn bonus
                        let passed_pawn_bonus = match color {
                            White => cond_val!((self.bb(Black, Pawn) & WHITE_PASSED_PAWN_MASKS[square]).is_empty(); evaluator.passed_pawn_bonus()[LOOKUP_RANK[square]]),
                            Black => cond_val!((self.bb(White, Pawn) & BLACK_PASSED_PAWN_MASKS[square]).is_empty(); evaluator.passed_pawn_bonus()[7 - LOOKUP_RANK[square]]),
                        };

                        let score = -stack_penalty - isolated_penalty + passed_pawn_bonus;// + supporting_pawn_bonus

                        (score + evaluator.pawn_scores_early()[piece_square_index], score + evaluator.pawn_scores_late()[piece_square_index])
                    },
                    Knight => {
                        // Mobility bonus
                        let move_cnt = (knight_attacks(square as u8) & !self.color_bb(color)).pop_count() as i16;
                        let (early_mob, late_mob) = (
                            evaluator.knight_mobility_bonus_early() * move_cnt,
                            evaluator.knight_mobility_bonus_late() * move_cnt
                        );

                        (
                            evaluator.knight_scores_early()[piece_square_index] + early_mob,
                            evaluator.knight_scores_late()[piece_square_index] + late_mob
                        )
                    },
                    Bishop => {
                        // Mobility bonus
                        let move_cnt = (d12_attacks(square as u8, self.all_occupancies) & !self.color_bb(color)).pop_count() as i16;
                        let (early_mob, late_mob) = (
                            evaluator.bishop_mobility_bonus_early() * move_cnt,
                            evaluator.bishop_mobility_bonus_late() * move_cnt
                        );

                        (
                            evaluator.bishop_scores_early()[piece_square_index] + early_mob,
                            evaluator.bishop_scores_late()[piece_square_index] + late_mob
                        )
                    },
                    Rook => {
                        // Semi open file bonus
                        let semi_open_bonus = cond_val!((self.bb(color, Pawn) & FILE_MASKS[square]).is_empty(); evaluator.semi_open_file_bonus());

                        // Open file bonus
                        let open_bonus = cond_val!(((self.bb(White, Pawn) | self.bb(Black, Pawn)) & FILE_MASKS[square]).is_empty(); evaluator.open_file_bonus());

                        // Mobility bonus
                        let move_cnt = (hv_attacks(square as u8, self.all_occupancies) & !self.color_bb(color)).pop_count() as i16;
                        let (early_mob, late_mob) = (
                            evaluator.rook_mobility_bonus_early() * move_cnt,
                            evaluator.rook_mobility_bonus_late() * move_cnt
                        );
                        
                        let score = semi_open_bonus + open_bonus;

                        (
                            score + evaluator.rook_scores_early()[piece_square_index] + early_mob,
                            score + evaluator.rook_scores_late()[piece_square_index] + late_mob
                        )
                    },
                    Queen => {
                        // Mobility bonus
                        let move_cnt = ((d12_attacks(square as u8, self.all_occupancies) | hv_attacks(square as u8, self.all_occupancies)) & !self.color_bb(color)).pop_count() as i16;
                        let (early_mob, late_mob) = (
                            evaluator.queen_mobility_bonus_early() * move_cnt,
                            evaluator.queen_mobility_bonus_late() * move_cnt
                        );

                        (
                            evaluator.queen_scores_early()[piece_square_index] + early_mob,
                            evaluator.queen_scores_late()[piece_square_index] + late_mob
                        )
                    },
                    King => {
                        // Semi open file penalty
                        let semi_open_penalty = cond_val!((self.bb(color, Pawn) & FILE_MASKS[square]).is_empty(); evaluator.semi_open_file_bonus());

                        // Open file penalty
                        let open_penalty = cond_val!(((self.bb(White, Pawn) | self.bb(Black, Pawn)) & FILE_MASKS[square]).is_empty(); evaluator.open_file_bonus());

                        // King safety bonus
                        let move_cnt = (king_attacks(square as u8) & self.color_bb(color)).pop_count() as i16;
                        let (early_safety, late_safety) = (
                            move_cnt * evaluator.protected_king_bonus_early(), 
                            move_cnt * evaluator.protected_king_bonus_late()
                        );

                        let score = -semi_open_penalty - open_penalty;

                        (
                            score + evaluator.king_scores_early()[piece_square_index] + early_safety,
                            score + evaluator.king_scores_late()[piece_square_index] + late_safety
                        )
                    },
                    Empty => unreachable!("Empty piece on board!"),
                });

                score = match color {
                    White => score + eval,
                    Black => score - eval,
                }
            }
        }

        let phase: f32 = {
            let p = TOTAL_PHASE - self.count_pieces(Pawn) * PAWN_PHASE
            - self.count_pieces(Knight) * KNIGHT_PHASE
            - self.count_pieces(Bishop) * BISHOP_PHASE
            - self.count_pieces(Rook) * ROOK_PHASE
            - self.count_pieces(Queen) * QUEEN_PHASE;

            ((p as f32) * 256. + ((TOTAL_PHASE as f32) / 2.)) / (TOTAL_PHASE as f32)
        };

        let eval = (((score.early() as f32 * (256. - phase)) + (score.late() as f32 * phase)) / 256.) as i16;

        if self.active_color.is_white() { eval } else { -eval }
    }
}

pub const EVAL_CONSTS: WeightArray = {
    let mut consts = [0; WEIGHT_COUNT];
    
    let mut i = 0;
    // material weights:
    { const_for!(a in 1..5 => {
            consts[i] = MATERIAL_WEIGHTS_EARLY[a];
            i += 1;
    }); }
    { const_for!(a in 1..5 => {
            consts[i] = MATERIAL_WEIGHTS_LATE[a];
            i += 1;
    }); }
    // pawn scores early:
    { const_for!(a in 0..64 => {
            consts[i] = PAWN_SCORES_EARLY[a];
            i += 1;
    }); }
    // pawn scores late:
    { const_for!(a in 0..64 => {
            consts[i] = PAWN_SCORES_LATE[a];
            i += 1;
    }); }
    // knight scores early:
    { const_for!(a in 0..64 => {
            consts[i] = KNIGHT_SCORES_EARLY[a];
            i += 1;
    }); }
    // knight scores late:
    { const_for!(a in 0..64 => {
            consts[i] = KNIGHT_SCORES_LATE[a];
            i += 1;
    }); }
    // bishop scores early:
    { const_for!(a in 0..64 => {
            consts[i] = BISHOP_SCORES_EARLY[a];
            i += 1;
    }); }
    // bishop scores late:
    { const_for!(a in 0..64 => {
            consts[i] = BISHOP_SCORES_LATE[a];
            i += 1;
    }); }
    // rook scores early:
    { const_for!(a in 0..64 => {
            consts[i] = ROOK_SCORES_EARLY[a];
            i += 1;
    }); }
    // rook scores late:
    { const_for!(a in 0..64 => {
            consts[i] = ROOK_SCORES_LATE[a];
            i += 1;
    }); }
    // queen scores early:
    { const_for!(a in 0..64 => {
            consts[i] = QUEEN_SCORES_EARLY[a];
            i += 1;
    }); }
    // queen scores late:
    { const_for!(a in 0..64 => {
            consts[i] = QUEEN_SCORES_LATE[a];
            i += 1;
    }); }
    // king scores early:
    { const_for!(a in 0..64 => {
            consts[i] = KING_SCORES_EARLY[a];
            i += 1;
    }); }
    // king scores late:
    { const_for!(a in 0..64 => {
            consts[i] = KING_SCORES_LATE[a];
            i += 1;
    }); }
    // stacked pawn penalty:
    consts[i] = STACKED_PAWN_PENALTY;
    i += 1;
    // isolated pawn penalty:
    consts[i] = ISOLATED_PAWN_PENALTY;
    i += 1;
    // passed pawn bonus:
    { const_for!(a in 0..8 => {
            consts[i] = PASSED_PAWN_BONUS[a];
            i += 1;
    }); }
    // semi open file bonus:
    consts[i] = SEMI_OPEN_FILE_BONUS;
    i += 1;
    // open file bonus:
    consts[i] = OPEN_FILE_BONUS;
    i += 1;

    // early protected king bonus:
    consts[i] = PROTECTED_KING_BONUS_EARLY;
    i += 1;
    // early knight mobility bonus:
    consts[i] = KNIGHT_MOBILITY_BONUS_EARLY;
    i += 1;
    // early bishop mobility bonus:
    consts[i] = BISHOP_MOBILITY_BONUS_EARLY;
    i += 1;
    // early rook mobility bonus:
    consts[i] = ROOK_MOBILITY_BONUS_EARLY;
    i += 1;
    // early queen mobility bonus:
    consts[i] = QUEEN_MOBILITY_BONUS_EARLY;
    i += 1;

    // late protected king bonus:
    consts[i] = PROTECTED_KING_BONUS_LATE;
    i += 1;
    // late knight mobility bonus:
    consts[i] = KNIGHT_MOBILITY_BONUS_LATE;
    i += 1;
    // late bishop mobility bonus:
    consts[i] = BISHOP_MOBILITY_BONUS_LATE;
    i += 1;
    // late rook mobility bonus:
    consts[i] = ROOK_MOBILITY_BONUS_LATE;
    i += 1;
    // late queen mobility bonus:
    consts[i] = QUEEN_MOBILITY_BONUS_LATE;
    
    consts
};

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