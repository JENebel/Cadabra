use std::fmt::Display;

use cadabra::*;

#[derive(Copy, Clone)]
pub struct TunerEvaluator {
    pub material_weights_early: [i16; 12],
    pub material_weights_late: [i16; 12],
    pub pawn_scores_early: [i16; 64],
    pub pawn_scores_late: [i16; 64],
    pub knight_scores_early: [i16; 64],
    pub knight_scores_late: [i16; 64],
    pub bishop_scores_early: [i16; 64],
    pub bishop_scores_late: [i16; 64],
    pub rook_scores_early: [i16; 64],
    pub rook_scores_late: [i16; 64],
    pub queen_scores_early: [i16; 64],
    pub queen_scores_late: [i16; 64],
    pub king_scores_early: [i16; 64],
    pub king_scores_late: [i16; 64],
    pub stacked_pawn_penalty: i16,
    pub isolated_pawn_penalty: i16,
    pub passed_pawn_bonus: [i16; 8],
    pub semi_open_file_bonus: i16,
    pub open_file_bonus: i16,
    pub protected_king_bonus_early: i16,
    pub knight_mobility_bonus_early: i16,
    pub bishop_mobility_bonus_early: i16,
    pub rook_mobility_bonus_early: i16,
    pub queen_mobility_bonus_early: i16,
    pub protected_king_bonus_late: i16,
    pub knight_mobility_bonus_late: i16,
    pub bishop_mobility_bonus_late: i16,
    pub rook_mobility_bonus_late: i16,
    pub queen_mobility_bonus_late: i16,
}

impl Evaluator for TunerEvaluator {
    fn material_weights_early(&self) -> &[i16; 12] { &self.material_weights_early }
    fn material_weights_late(&self) -> &[i16; 12] { &self.material_weights_late }
    fn pawn_scores_early(&self) -> &[i16; 64] { &self.pawn_scores_early }
    fn pawn_scores_late(&self) -> &[i16; 64] { &self.pawn_scores_late }
    fn knight_scores_early(&self) -> &[i16; 64] { &self.knight_scores_early }
    fn knight_scores_late(&self) -> &[i16; 64] { &self.knight_scores_late }
    fn bishop_scores_early(&self) -> &[i16; 64] { &self.bishop_scores_early }
    fn bishop_scores_late(&self) -> &[i16; 64] { &self.bishop_scores_late }
    fn rook_scores_early(&self) -> &[i16; 64] { &self.rook_scores_early }
    fn rook_scores_late(&self) -> &[i16; 64] { &self.rook_scores_late }
    fn queen_scores_early(&self) -> &[i16; 64] { &self.queen_scores_early }
    fn queen_scores_late(&self) -> &[i16; 64] { &self.queen_scores_late }
    fn king_scores_early(&self) -> &[i16; 64] { &self.king_scores_early }
    fn king_scores_late(&self) -> &[i16; 64] { &self.king_scores_late }
    fn stacked_pawn_penalty(&self) -> i16 { self.stacked_pawn_penalty }
    fn isolated_pawn_penalty(&self) -> i16 { self.isolated_pawn_penalty }
    fn passed_pawn_bonus(&self) -> &[i16; 8] { &self.passed_pawn_bonus }
    fn semi_open_file_bonus(&self) -> i16 { self.semi_open_file_bonus }
    fn open_file_bonus(&self) -> i16 { self.open_file_bonus }
    fn protected_king_bonus_early(&self) -> i16 { self.protected_king_bonus_early }
    fn knight_mobility_bonus_early(&self) -> i16 { self.knight_mobility_bonus_early }
    fn bishop_mobility_bonus_early(&self) -> i16 { self.bishop_mobility_bonus_early }
    fn rook_mobility_bonus_early(&self) -> i16 { self.rook_mobility_bonus_early }
    fn queen_mobility_bonus_early(&self) -> i16 { self.queen_mobility_bonus_early }
    fn protected_king_bonus_late(&self) -> i16 { self.protected_king_bonus_late }
    fn knight_mobility_bonus_late(&self) -> i16 { self.knight_mobility_bonus_late }
    fn bishop_mobility_bonus_late(&self) -> i16 { self.bishop_mobility_bonus_late }
    fn rook_mobility_bonus_late(&self) -> i16 { self.rook_mobility_bonus_late }
    fn queen_mobility_bonus_late(&self) -> i16 { self.queen_mobility_bonus_late }
}

impl Default for TunerEvaluator {
    #[inline(always)]
    fn default() -> Self {
        TunerEvaluator {
            material_weights_early: MATERIAL_WEIGHTS_EARLY,
            material_weights_late: MATERIAL_WEIGHTS_LATE,
            pawn_scores_early: PAWN_SCORES_EARLY,
            pawn_scores_late: PAWN_SCORES_LATE,
            knight_scores_early: KNIGHT_SCORES_EARLY,
            knight_scores_late: KNIGHT_SCORES_LATE,
            bishop_scores_early: BISHOP_SCORES_EARLY,
            bishop_scores_late: BISHOP_SCORES_LATE,
            rook_scores_early: ROOK_SCORES_EARLY,
            rook_scores_late: ROOK_SCORES_LATE,
            queen_scores_early: QUEEN_SCORES_EARLY,
            queen_scores_late: QUEEN_SCORES_LATE,
            king_scores_early: KING_SCORES_EARLY,
            king_scores_late: KING_SCORES_LATE,
            stacked_pawn_penalty: STACKED_PAWN_PENALTY,
            isolated_pawn_penalty: ISOLATED_PAWN_PENALTY,
            passed_pawn_bonus: PASSED_PAWN_BONUS,
            semi_open_file_bonus: SEMI_OPEN_FILE_BONUS,
            open_file_bonus: OPEN_FILE_BONUS,
            protected_king_bonus_early: PROTECTED_KING_BONUS_EARLY,
            knight_mobility_bonus_early: KNIGHT_MOBILITY_BONUS_EARLY,
            bishop_mobility_bonus_early: BISHOP_MOBILITY_BONUS_EARLY,
            rook_mobility_bonus_early: ROOK_MOBILITY_BONUS_EARLY,
            queen_mobility_bonus_early: QUEEN_MOBILITY_BONUS_EARLY,
            protected_king_bonus_late: PROTECTED_KING_BONUS_LATE,
            knight_mobility_bonus_late: KNIGHT_MOBILITY_BONUS_LATE,
            bishop_mobility_bonus_late: BISHOP_MOBILITY_BONUS_LATE,
            rook_mobility_bonus_late: ROOK_MOBILITY_BONUS_LATE,
            queen_mobility_bonus_late: QUEEN_MOBILITY_BONUS_LATE,
        }
    }
}

impl TunerEvaluator {
    pub fn get_weights(&self) -> WeightArray {
        let mut weights = Vec::new();
        weights.extend_from_slice(&self.material_weights_early[1..5]);
        weights.extend_from_slice(&self.material_weights_late[1..5]);
        weights.extend_from_slice(&self.pawn_scores_early);
        weights.extend_from_slice(&self.pawn_scores_late);
        weights.extend_from_slice(&self.knight_scores_early);
        weights.extend_from_slice(&self.knight_scores_late);
        weights.extend_from_slice(&self.bishop_scores_early);
        weights.extend_from_slice(&self.bishop_scores_late);
        weights.extend_from_slice(&self.rook_scores_early);
        weights.extend_from_slice(&self.rook_scores_late);
        weights.extend_from_slice(&self.queen_scores_early);
        weights.extend_from_slice(&self.queen_scores_late);
        weights.extend_from_slice(&self.king_scores_early);
        weights.extend_from_slice(&self.king_scores_late);
        weights.push(self.stacked_pawn_penalty);
        weights.push(self.isolated_pawn_penalty);
        weights.extend_from_slice(&self.passed_pawn_bonus);
        weights.push(self.semi_open_file_bonus);
        weights.push(self.open_file_bonus);
        weights.push(self.protected_king_bonus_early);
        weights.push(self.knight_mobility_bonus_early);
        weights.push(self.bishop_mobility_bonus_early);
        weights.push(self.rook_mobility_bonus_early);
        weights.push(self.queen_mobility_bonus_early);
        weights.push(self.protected_king_bonus_late);
        weights.push(self.knight_mobility_bonus_late);
        weights.push(self.bishop_mobility_bonus_late);
        weights.push(self.rook_mobility_bonus_late);
        weights.push(self.queen_mobility_bonus_late);
        weights.into_iter().map(|x| x as i16).collect::<Vec<_>>().try_into().unwrap()
    }


    pub fn from_weights(weights: WeightArray) -> Self {
        let mut material_weights_early = [0; 12];
        material_weights_early[0] = 100;
        material_weights_early[1] = weights[0];
        material_weights_early[2] = weights[1];
        material_weights_early[3] = weights[2];
        material_weights_early[4] = weights[3];
        material_weights_early[6] = -100;
        material_weights_early[7] = -weights[0];
        material_weights_early[8] = -weights[1];
        material_weights_early[9] = -weights[2];
        material_weights_early[10] = -weights[3];

        let mut material_weights_late = [0; 12];
        material_weights_late[0] = 100;
        material_weights_late[1] = weights[4];
        material_weights_late[2] = weights[5];
        material_weights_late[3] = weights[6];
        material_weights_late[4] = weights[7];
        material_weights_late[6] = -100;
        material_weights_late[7] = -weights[4];
        material_weights_late[8] = -weights[5];
        material_weights_late[9] = -weights[6];
        material_weights_late[10] = -weights[7];

        let mut weights = weights.into_iter().skip(8);

        fn take_n<const N: usize>(iter: &mut dyn Iterator<Item = i16>) -> [i16; N] {
            let mut arr = [0; N];
            for i in 0..N {
                arr[i] = iter.next().unwrap();
            }
            arr
        }

        TunerEvaluator {
            material_weights_early,
            material_weights_late,
            pawn_scores_early: take_n::<64>(&mut weights),
            pawn_scores_late: take_n::<64>(&mut weights),
            knight_scores_early: take_n::<64>(&mut weights),
            knight_scores_late: take_n::<64>(&mut weights),
            bishop_scores_early: take_n::<64>(&mut weights),
            bishop_scores_late: take_n::<64>(&mut weights),
            rook_scores_early: take_n::<64>(&mut weights),
            rook_scores_late: take_n::<64>(&mut weights),
            queen_scores_early: take_n::<64>(&mut weights),
            queen_scores_late: take_n::<64>(&mut weights),
            king_scores_early: take_n::<64>(&mut weights),
            king_scores_late: take_n::<64>(&mut weights),
            stacked_pawn_penalty: weights.next().unwrap(),
            isolated_pawn_penalty: weights.next().unwrap(),
            passed_pawn_bonus: take_n::<8>(&mut weights),
            semi_open_file_bonus: weights.next().unwrap(),
            open_file_bonus: weights.next().unwrap(),
            protected_king_bonus_early: weights.next().unwrap(),
            knight_mobility_bonus_early: weights.next().unwrap(),
            bishop_mobility_bonus_early: weights.next().unwrap(),
            rook_mobility_bonus_early: weights.next().unwrap(),
            queen_mobility_bonus_early: weights.next().unwrap(),
            protected_king_bonus_late: weights.next().unwrap(),
            knight_mobility_bonus_late: weights.next().unwrap(),
            bishop_mobility_bonus_late: weights.next().unwrap(),
            rook_mobility_bonus_late: weights.next().unwrap(),
            queen_mobility_bonus_late: weights.next().unwrap(),
        }
    }
}

impl Display for TunerEvaluator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //writeln!(f, "/* Generated {} */", Local::now().naive_local())?;
        write!(f, "pub const MATERIAL_WEIGHTS_EARLY: [i16; 12] = {:?};\n", self.material_weights_early)?;
        write!(f, "pub const MATERIAL_WEIGHTS_LATE: [i16; 12] = {:?};\n", self.material_weights_late)?;
        write!(f, "\npub const STACKED_PAWN_PENALTY: i16 = {};\n", self.stacked_pawn_penalty)?;
        write!(f, "pub const ISOLATED_PAWN_PENALTY: i16 = {};\n", self.isolated_pawn_penalty)?;
        write!(f, "pub const PASSED_PAWN_BONUS: [i16; 8] = {:?};\n", self.passed_pawn_bonus)?;
        write!(f, "pub const SEMI_OPEN_FILE_BONUS: i16 = {};\n", self.semi_open_file_bonus)?;
        write!(f, "pub const OPEN_FILE_BONUS: i16 = {};\n", self.open_file_bonus)?;
        write!(f, "\npub const PROTECTED_KING_BONUS_EARLY: i16 = {};\n", self.protected_king_bonus_early)?;
        write!(f, "pub const KNIGHT_MOBILITY_BONUS_EARLY: i16 = {};\n", self.knight_mobility_bonus_early)?;
        write!(f, "pub const BISHOP_MOBILITY_BONUS_EARLY: i16 = {};\n", self.bishop_mobility_bonus_early)?;
        write!(f, "pub const ROOK_MOBILITY_BONUS_EARLY: i16 = {};\n", self.rook_mobility_bonus_early)?;
        write!(f, "pub const QUEEN_MOBILITY_BONUS_EARLY: i16 = {};\n", self.queen_mobility_bonus_early)?;
        write!(f, "\npub const PROTECTED_KING_BONUS_LATE: i16 = {};\n", self.protected_king_bonus_late)?;
        write!(f, "pub const KNIGHT_MOBILITY_BONUS_LATE: i16 = {};\n", self.knight_mobility_bonus_late)?;
        write!(f, "pub const BISHOP_MOBILITY_BONUS_LATE: i16 = {};\n", self.bishop_mobility_bonus_late)?;
        write!(f, "pub const ROOK_MOBILITY_BONUS_LATE: i16 = {};\n", self.rook_mobility_bonus_late)?;
        write!(f, "pub const QUEEN_MOBILITY_BONUS_LATE: i16 = {};\n", self.queen_mobility_bonus_late)?;
        write!(f, "pub const PAWN_SCORES_EARLY: [i16; 64] = \n")?;
        display_pst(self.pawn_scores_early, f)?;
        write!(f, "pub const PAWN_SCORES_LATE: [i16; 64] = \n")?;
        display_pst(self.pawn_scores_late, f)?;
        write!(f, "pub const KNIGHT_SCORES_EARLY: [i16; 64] = \n")?;
        display_pst(self.knight_scores_early, f)?;
        write!(f, "pub const KNIGHT_SCORES_LATE: [i16; 64] = \n")?;
        display_pst(self.knight_scores_late, f)?;
        write!(f, "pub const BISHOP_SCORES_EARLY: [i16; 64] = \n")?;
        display_pst(self.bishop_scores_early, f)?;
        write!(f, "pub const BISHOP_SCORES_LATE: [i16; 64] = \n")?;
        display_pst(self.bishop_scores_late, f)?;
        write!(f, "pub const ROOK_SCORES_EARLY: [i16; 64] = \n")?;
        display_pst(self.rook_scores_early, f)?;
        write!(f, "pub const ROOK_SCORES_LATE: [i16; 64] = \n")?;
        display_pst(self.rook_scores_late, f)?;
        write!(f, "pub const QUEEN_SCORES_EARLY: [i16; 64] = \n")?;
        display_pst(self.queen_scores_early, f)?;
        write!(f, "pub const QUEEN_SCORES_LATE: [i16; 64] = \n")?;
        display_pst(self.queen_scores_late, f)?;
        write!(f, "pub const KING_SCORES_EARLY: [i16; 64] = \n")?;
        display_pst(self.king_scores_early, f)?;
        write!(f, "pub const KING_SCORES_LATE: [i16; 64] = \n")?;
        display_pst(self.king_scores_late, f)?;

        Ok(())
    }
}

fn display_pst(table: [i16; 64], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[\n")?;
    for i in 0..8 {
        write!(f, "    ")?;
        for j in 0..8 {
            let s = format!("{}", table[i * 8 + j]);
            write!(f, "{},{}", &s, " ".repeat(5 - s.len()))?;
        }
        write!(f, "\n")?;
    }
    writeln!(f, "];\n")
}