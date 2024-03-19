use super::eval_consts::*;

pub trait Evaluator {
    fn material_weights_early(&self) -> &[i16; 12];
    fn material_weights_late(&self) -> &[i16; 12];
    fn pawn_scores_early(&self) -> &[i16; 64];
    fn pawn_scores_late(&self) -> &[i16; 64];
    fn knight_scores_early(&self) -> &[i16; 64];
    fn knight_scores_late(&self) -> &[i16; 64];
    fn bishop_scores_early(&self) -> &[i16; 64];
    fn bishop_scores_late(&self) -> &[i16; 64];
    fn rook_scores_early(&self) -> &[i16; 64];
    fn rook_scores_late(&self) -> &[i16; 64];
    fn queen_scores_early(&self) -> &[i16; 64];
    fn queen_scores_late(&self) -> &[i16; 64];
    fn king_scores_early(&self) -> &[i16; 64];
    fn king_scores_late(&self) -> &[i16; 64];
    fn stacked_pawn_penalty(&self) -> i16;
    fn isolated_pawn_penalty(&self) -> i16;
    fn passed_pawn_bonus(&self) -> &[i16; 8];
    fn semi_open_file_bonus(&self) -> i16;
    fn open_file_bonus(&self) -> i16;
    fn protected_king_bonus_early(&self) -> i16;
    fn knight_mobility_bonus_early(&self) -> i16;
    fn bishop_mobility_bonus_early(&self) -> i16;
    fn rook_mobility_bonus_early(&self) -> i16;
    fn queen_mobility_bonus_early(&self) -> i16;
    fn protected_king_bonus_late(&self) -> i16;
    fn knight_mobility_bonus_late(&self) -> i16;
    fn bishop_mobility_bonus_late(&self) -> i16;
    fn rook_mobility_bonus_late(&self) -> i16;
    fn queen_mobility_bonus_late(&self) -> i16;
}

pub const CONST_EVALUATOR: ConstantEvaluator = ConstantEvaluator { };

#[derive(Clone, Copy)]
pub struct ConstantEvaluator { }

impl Evaluator for ConstantEvaluator {
    #[inline(always)]
    fn material_weights_early(&self) -> &[i16; 12] { &MATERIAL_WEIGHTS_EARLY }
    #[inline(always)]
    fn material_weights_late(&self) -> &[i16; 12] { &MATERIAL_WEIGHTS_LATE }
    #[inline(always)]
    fn pawn_scores_early(&self) -> &[i16; 64] { &PAWN_SCORES_EARLY }
    #[inline(always)]
    fn pawn_scores_late(&self) -> &[i16; 64] { &PAWN_SCORES_LATE }
    #[inline(always)]
    fn knight_scores_early(&self) -> &[i16; 64] { &KNIGHT_SCORES_EARLY }
    #[inline(always)]
    fn knight_scores_late(&self) -> &[i16; 64] { &KNIGHT_SCORES_LATE }
    #[inline(always)]
    fn bishop_scores_early(&self) -> &[i16; 64] { &BISHOP_SCORES_EARLY }
    #[inline(always)]
    fn bishop_scores_late(&self) -> &[i16; 64] { &BISHOP_SCORES_LATE }
    #[inline(always)]
    fn rook_scores_early(&self) -> &[i16; 64] { &ROOK_SCORES_EARLY }
    #[inline(always)]
    fn rook_scores_late(&self) -> &[i16; 64] { &ROOK_SCORES_LATE }
    #[inline(always)]
    fn queen_scores_early(&self) -> &[i16; 64] { &QUEEN_SCORES_EARLY }
    #[inline(always)]
    fn queen_scores_late(&self) -> &[i16; 64] { &QUEEN_SCORES_LATE }
    #[inline(always)]
    fn king_scores_early(&self) -> &[i16; 64] { &KING_SCORES_EARLY }
    #[inline(always)]
    fn king_scores_late(&self) -> &[i16; 64] { &KING_SCORES_LATE }
    #[inline(always)]
    fn stacked_pawn_penalty(&self) -> i16 { STACKED_PAWN_PENALTY }
    #[inline(always)]
    fn isolated_pawn_penalty(&self) -> i16 { ISOLATED_PAWN_PENALTY }
    #[inline(always)]
    fn passed_pawn_bonus(&self) -> &[i16; 8] { &PASSED_PAWN_BONUS }
    #[inline(always)]
    fn semi_open_file_bonus(&self) -> i16 { SEMI_OPEN_FILE_BONUS }
    #[inline(always)]
    fn open_file_bonus(&self) -> i16 { OPEN_FILE_BONUS }
    #[inline(always)]
    fn protected_king_bonus_early(&self) -> i16 { PROTECTED_KING_BONUS_EARLY }
    #[inline(always)]
    fn knight_mobility_bonus_early(&self) -> i16 { KNIGHT_MOBILITY_BONUS_EARLY }
    #[inline(always)]
    fn bishop_mobility_bonus_early(&self) -> i16 { BISHOP_MOBILITY_BONUS_EARLY }
    #[inline(always)]
    fn rook_mobility_bonus_early(&self) -> i16 { ROOK_MOBILITY_BONUS_EARLY }
    #[inline(always)]
    fn queen_mobility_bonus_early(&self) -> i16 { QUEEN_MOBILITY_BONUS_EARLY }
    #[inline(always)]
    fn protected_king_bonus_late(&self) -> i16 { PROTECTED_KING_BONUS_LATE }
    #[inline(always)]
    fn knight_mobility_bonus_late(&self) -> i16 { KNIGHT_MOBILITY_BONUS_LATE }
    #[inline(always)]
    fn bishop_mobility_bonus_late(&self) -> i16 { BISHOP_MOBILITY_BONUS_LATE }
    #[inline(always)]
    fn rook_mobility_bonus_late(&self) -> i16 { ROOK_MOBILITY_BONUS_LATE }
    #[inline(always)]
    fn queen_mobility_bonus_late(&self) -> i16 { QUEEN_MOBILITY_BONUS_LATE }
}