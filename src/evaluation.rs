use crate::{position::*, bitboard::*, attack_tables::*, constants::*, definitions::*};

////////////////////////////////////////
/// General                          ///
////////////////////////////////////////

const STACKED_PAWN_PENALTY: i32 = -10;
const ISOLATED_PAWN_PENALTY: i32 = -10;
const PASSED_WHITE_PAWN_BONUS: [i32; 8] = [ 0, 10, 30, 50, 75, 100, 150, 200 ];
const PASSED_BLACK_PAWN_BONUS: [i32; 8] = [ 200, 150, 100, 75, 50, 30, 10, 0 ]; 
const SEMI_OPEN_FILE_SCORE: i32 = 10;
const OPEN_FILE_SCORE: i32 = 15;
const PROTECTED_KING_BONUS: i32 = 5;

impl Position {
    pub fn evaluate(&self) -> i32 {
        /*let mut score: i32 = 0;

        let mut stacked_pawns;

        for bb in 0..12 {
            let mut board = self.get_bitboard(bb);
            while let Some(square) = board.extract_bit() {
                score += MATERIAL_WEIGHTS[bb];
                match bb {
                    //White pawns
                    0  => {
                        score += PAWN_SCORES[square as usize];

                        //Stacked pawn penalty
                        stacked_pawns = self.get_piece_bitboard(WhitePawn)
                                            .and(Bitboard::from(FILE_MASKS[square as usize]))
                                            .count();
                        if stacked_pawns > 1 {
                            score += stacked_pawns as i32 * STACKED_PAWN_PENALTY;
                        }

                        //Isolated pawn penalty
                        if self.get_piece_bitboard(WhitePawn)
                                .and(Bitboard::from(ISOLATED_MASKS[square as usize]))
                                .is_empty() {
                            score += ISOLATED_PAWN_PENALTY;
                        }

                        //Passed pawn penalty
                        if self.get_piece_bitboard(BlackPawn)
                            .and(Bitboard::from(WHITE_PASSED_PAWN_MASKS[square as usize]))
                            .is_empty() {
                            score += PASSED_WHITE_PAWN_BONUS[LOOKUP_RANK[square as usize]];
                        }
                    },
                    //White knight
                    1  => {
                        score += KNIGHT_SCORES[square as usize];

                        //Mobility
                        //score += (get_knight_attack_table(square).pop_count() - KNIGHT_UNIT) as i32 * KNIGHT_MOB;
                        score += get_knight_attack_table(square).count() as i32;
                    },
                    //White bishops
                    2  => {
                        score += BISHOP_SCORES[square as usize];

                        //Mobility
                        //score += (get_bishop_attack_table(square, self.all_occupancies).pop_count() - BISHOP_UNIT) as i32 * BISHOP_MOB;
                        score += (get_bishop_attack_table(square, self.all_occupancies).count()) as i32;

                    },
                    //White Rooks
                    3  => {
                        score += ROOK_SCORES[square as usize];

                        //Semi open file bonus
                        if self.get_piece_bitboard(WhitePawn)
                            .and(Bitboard::from(FILE_MASKS[square as usize]))
                            .is_empty() {
                            score += SEMI_OPEN_FILE_SCORE;
                        }

                        //Open file bonus
                        if (self.get_piece_bitboard(WhitePawn)
                                .or(self.get_piece_bitboard(BlackPawn)))
                                    .and(Bitboard::from(FILE_MASKS[square as usize])).is_empty() {
                            score += OPEN_FILE_SCORE;
                        }

                        //Mobility
                        //score += (get_rook_attack_table(square, self.all_occupancies).pop_count() - ROOK_UNIT) as i32 * ROOK_MOB;
                        score += get_rook_attack_table(square, self.all_occupancies).count() as i32;
                    },
                    //White queen
                    4 => {
                        //Mobility
                        //score += ((get_queen_attack_table(square, self.all_occupancies).pop_count() - QUEEN_UNIT) as f32 * QUEEN_MOB) as i32;
                        score += get_queen_attack_table(square, self.all_occupancies).count() as i32;
                    },
                    //White king
                    5  => {
                        score += KING_SCORES[square as usize];

                        //Semi open file penalty
                        if self.get_piece_bitboard(WhitePawn)
                            .and(Bitboard::from(FILE_MASKS[square as usize]))
                            .is_empty() {
                            score -= SEMI_OPEN_FILE_SCORE;
                        }

                        //Open file penalty
                        if (self.get_piece_bitboard(WhitePawn)
                                .or(self.get_piece_bitboard(BlackPawn)))
                                    .and(Bitboard::from(FILE_MASKS[square as usize])).is_empty() {
                            score -= OPEN_FILE_SCORE;
                        }

                        //King safety
                        score += get_king_attack_table(square).and(self.white_occupancies).count() as i32 * PROTECTED_KING_BONUS;
                    },
                    //Black pawns
                    6  => {
                        score -= PAWN_SCORES[MIRRORED[square as usize]];
                        
                        //Stacked pawn penalty
                        stacked_pawns = self.get_piece_bitboard(BlackPawn)
                                            .and(Bitboard::from(FILE_MASKS[square as usize]))
                                            .count();
                        if stacked_pawns > 1 {
                            score -= stacked_pawns as i32 * STACKED_PAWN_PENALTY;
                        }

                        //Isolated pawn penalty
                        if self.get_piece_bitboard(BlackPawn)
                            .and(Bitboard::from(ISOLATED_MASKS[square as usize]))
                            .is_empty() {
                            score -= ISOLATED_PAWN_PENALTY;
                        }

                        //Passed pawn penalty
                        if self.get_piece_bitboard(WhitePawn)
                            .and(Bitboard::from(BLACK_PASSED_PAWN_MASKS[square as usize]))
                            .is_empty() {
                            score -= PASSED_BLACK_PAWN_BONUS[LOOKUP_RANK[square as usize]];
                        }
                    },
                    //Black knight
                    7  => {
                        score -= KNIGHT_SCORES[MIRRORED[square as usize]];

                        //Mobility
                        //score -= (get_knight_attack_table(square).pop_count() - KNIGHT_UNIT) as i32 * KNIGHT_MOB;
                        score -= get_knight_attack_table(square).count() as i32;
                    },
                    //Black bishop
                    8  => {
                        score -= BISHOP_SCORES[MIRRORED[square as usize]];

                        //Mobility
                        //score -= (get_bishop_attack_table(square, self.all_occupancies).pop_count() - BISHOP_UNIT) as i32 * BISHOP_MOB;
                        score -= get_bishop_attack_table(square, self.all_occupancies).count() as i32;
                    },
                    //Black rooks
                    9  => {
                        score -= ROOK_SCORES[MIRRORED[square as usize]];

                        //Semi open file bonus
                        if self.get_piece_bitboard(BlackPawn)
                            .and(Bitboard::from(FILE_MASKS[square as usize]))
                            .is_empty() {
                            score -= SEMI_OPEN_FILE_SCORE;
                        }

                        //Open file bonus
                        if (self.get_piece_bitboard(BlackPawn)
                                .or(self.get_piece_bitboard(WhitePawn)))
                                    .and(Bitboard::from(FILE_MASKS[square as usize])).is_empty() {
                            score -= OPEN_FILE_SCORE;
                        }

                        //Mobility
                        //score -= (get_rook_attack_table(square, self.all_occupancies).pop_count() - ROOK_UNIT) as i32 * ROOK_MOB;
                        score -= get_rook_attack_table(square, self.all_occupancies).count() as i32;
                    },
                    //Black queen
                    10 => {
                        //Mobility
                        //score -= ((get_queen_attack_table(square, self.all_occupancies).pop_count() - QUEEN_UNIT) as f32 * QUEEN_MOB) as i32;
                        score -= get_queen_attack_table(square, self.all_occupancies).count() as i32;
                    }
                    //Black king
                    11 => {
                        score -= KING_SCORES[MIRRORED[square as usize]];

                        //Semi open file penalty
                        if self.get_piece_bitboard(BlackPawn)
                            .and(Bitboard::from(FILE_MASKS[square as usize]))
                            .is_empty() {
                            score += SEMI_OPEN_FILE_SCORE;
                        }

                        //Open file penalty
                        if (self.get_piece_bitboard(BlackPawn)
                                .or(self.get_piece_bitboard(WhitePawn)))
                                    .and(Bitboard::from(FILE_MASKS[square as usize])).is_empty() {
                            score += OPEN_FILE_SCORE;
                        }

                        //King safety
                        score -= get_king_attack_table(square).and(self.black_occupancies).count() as i32 * PROTECTED_KING_BONUS;
                    },
                    _ => unreachable!()
                };
            }
        }

        if self.active_color == Color::White { score } else { -score } // Colud avoid branching here*/

        10
    }
}
