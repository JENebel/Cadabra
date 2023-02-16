use super::*;

use PieceType::*;
use MoveType::*;
use Color::*;

// Macros to expand const generics for move generation
/*macro_rules! generate_moves_match_has_enpassant {
    ($move_gen: expr, $pos: expr, $is_quiescence: expr, $is_white: expr, $is_sorting: expr) => {
        match $pos.enpassant_square.is_some() {
            true =>  $move_gen.generate_moves::<$is_quiescence, $is_white, $is_sorting, true>($pos),
            false => $move_gen.generate_moves::<$is_quiescence, $is_white, $is_sorting, false>($pos),
        }
    };
}
macro_rules! generate_moves_match_is_sorting {
    ($move_gen: expr, $pos: expr, $is_quiescence: expr, $is_white: expr, $is_sorting: expr) => {
        match $is_sorting {
            true =>  generate_moves_match_has_enpassant!($move_gen, $pos, $is_quiescence, $is_white, true),
            false => generate_moves_match_has_enpassant!($move_gen, $pos, $is_quiescence, $is_white, false),
        }
    };
}
macro_rules! generate_moves_match_color {
    ($move_gen: expr, $pos: expr, $is_quiescence: expr, $is_sorting: expr) => {
        match $pos.active_color {
            Color::White => generate_moves_match_is_sorting!($move_gen, $pos, $is_quiescence, true, $is_sorting),
            Color::Black => generate_moves_match_is_sorting!($move_gen, $pos, $is_quiescence, false, $is_sorting),
        }
    };
}
macro_rules! generate_moves_match_move_types {
    ($move_gen: expr, $pos: expr, $is_quiescence: expr, $is_sorting: expr) => {
        match $is_quiescence {
            true =>  generate_moves_match_color!($move_gen, $pos, true, $is_sorting),
            false => generate_moves_match_color!($move_gen, $pos, false, $is_sorting),
        }
    };
}
macro_rules! generate_moves {
    ($move_gen: expr, $pos: expr, $is_quiescence: expr, $is_sorting: expr) => {
        // Match color
        generate_moves_match_move_types!($move_gen, $pos, $is_quiescence, $is_sorting)
    };
}*/

macro_rules! generate_pawn_captures {
    ($pos: expr, $move_list: expr, $has_enpassant_sq: expr, $from_sq: expr, $check_mask: expr, $pin_mask: expr) => {
        match $has_enpassant_sq {
            true =>  $pos.generate_pawn_captures::<true>($move_list, $from_sq, $check_mask, $pin_mask),
            false => $pos.generate_pawn_captures::<false>($move_list, $from_sq, $check_mask, $pin_mask)
        }
    };
}

pub struct MoveList {
    insert_index: usize,
    extract_index: usize,

    move_list: [Move; 128],
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            insert_index: 0,
            extract_index: 0,
            move_list: [Default::default(); 128],
        }
    }

    /// Gets the amount of moves stored in the list
    pub fn len(&self) -> usize {
        self.insert_index
    }

    /// Extracts a new move into the list
    pub fn insert(&mut self, new_move: Move) {
        self.move_list[self.insert_index] = new_move;
        self.insert_index += 1;
    }

    /// Extracts the best move in the list
    #[inline(always)]
    pub fn next_best(&mut self) -> Option<Move> {
        if self.extract_index == self.insert_index {
            return None
        };

        let mut best_index = self.extract_index;

        for i in self.extract_index..self.insert_index {
            let best_score = self.move_list[best_index].score;
            let score = self.move_list[i].score;

            if score > best_score {
                best_index = i
            }
        }

        self.move_list.swap(self.extract_index, best_index);

        let extracted = self.move_list[self.extract_index];
        self.extract_index += 1;

        Some(extracted)
    }
}

impl Iterator for MoveList {
    type Item = Move;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.extract_index == self.insert_index {
            return None
        };

        let extracted = self.move_list[self.extract_index];
        self.extract_index += 1;
        Some(extracted)
    }
}

impl Position {
    /// Generate more legal moves for the position
    #[inline(always)]
    pub fn generate_moves(&self) -> MoveList {
        let mut move_list = MoveList::new();
        let color = self.active_color;

        let check_mask = self.generate_check_mask(color);

        // If in double check, only king can move
        let checkers = (check_mask & self.color_bb(opposite_color(color))).count();
        if (!check_mask).is_not_empty() && checkers > 1 {
            self.generate_king_moves::<false>(&mut move_list);
            return move_list
        }

        // Generate the pin masks
        let hv_pin = self.generate_hv_pin_mask(color);
        let d12_pin = self.generate_d12_pin_mask(color);
        let opp_or_empty = !self.color_bb(color);
        
        // Pawn moves
        self.generate_pawn_moves(&mut move_list, check_mask, hv_pin, d12_pin);

        // Knight moves. Only unpinned can move
        let mut unpinned_knights = self.bb(color, Knight) & !(hv_pin | d12_pin);
        while let Some(sq) = unpinned_knights.extract_bit() {
            let seen = knight_attacks(sq);
            let legal = seen & opp_or_empty & check_mask;
            self.add_normal_moves(&mut move_list, sq, legal, Knight)
        }

        // Rook moves
        let rooks = self.bb(color, Rook);
        let mut pinned_rooks = rooks & hv_pin;
        while let Some(sq) = pinned_rooks.extract_bit() {
            let seen = hv_attacks(sq, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask & hv_pin;
            self.add_normal_moves(&mut move_list, sq, legal, Rook)
        }

        let mut unpinned_rooks = rooks & !(hv_pin | d12_pin);
        while let Some(sq) = unpinned_rooks.extract_bit() {
            let seen = hv_attacks(sq, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask;
            self.add_normal_moves(&mut move_list, sq, legal, Rook)
        }

        // Bishop moves
        let bishops = self.bb(color, Bishop);
        let mut pinned_bishops = bishops & d12_pin;
        while let Some(sq) = pinned_bishops.extract_bit() {
            let seen = d12_attacks(sq, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask & d12_pin;
            self.add_normal_moves(&mut move_list, sq, legal, Bishop)
        }

        let mut unpinned_bishops = bishops & !(hv_pin | d12_pin);
        while let Some(sq) = unpinned_bishops.extract_bit() {
            let seen = d12_attacks(sq, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask;
            self.add_normal_moves(&mut move_list, sq, legal, Bishop)
        }

        // Queen moves
        let queens = self.bb(color, Queen);
        let mut hv_pinned_queens = queens & hv_pin;
        while let Some(sq) = hv_pinned_queens.extract_bit() {
            let seen = hv_attacks(sq, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask & hv_pin;
            self.add_normal_moves(&mut move_list, sq, legal, Queen)
        }

        let mut d12_pinned_queens = queens & d12_pin;
        while let Some(sq) = d12_pinned_queens.extract_bit() {
            let seen = d12_attacks(sq, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask & d12_pin;
            self.add_normal_moves(&mut move_list, sq, legal, Queen)
        }

        let mut unpinned_queens = queens & !(hv_pin | d12_pin);
        while let Some(sq) = unpinned_queens.extract_bit() {
            let seen = d12_attacks(sq, self.all_occupancies) | hv_attacks(sq, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask;
            self.add_normal_moves(&mut move_list, sq, legal, Queen)
        }

        // King moves
        self.generate_king_moves::<true>(&mut move_list);

        move_list
    }

    #[inline(always)]
    fn generate_quiet_pawn_moves(&self, move_list: &mut MoveList, from_sq: u8, valid_mask: Bitboard) {
        let color = self.active_color;

        let fwd_sq = if color.is_white() {
            from_sq - 8
        } else {
            from_sq + 8
        };

        // If square in front free
        if !self.all_occupancies.get_bit(fwd_sq) {
            let last_rank = if color.is_white() {
                TOP_RANK
            } else {
                BOTTOM_RANK
            };

            // Determine if promotion
            if last_rank.get_bit(fwd_sq) {
                if valid_mask.get_bit(fwd_sq) {
                    move_list.insert(Move::new_promotion(from_sq, fwd_sq, Queen,  false));
                    move_list.insert(Move::new_promotion(from_sq, fwd_sq, Rook,   false));
                    move_list.insert(Move::new_promotion(from_sq, fwd_sq, Bishop, false));
                    move_list.insert(Move::new_promotion(from_sq, fwd_sq, Knight, false));
                }
            }
            else {
                // Normal move
                if valid_mask.get_bit(fwd_sq) {
                    move_list.insert(Move::new_normal(from_sq, fwd_sq, Pawn, false))
                }

                // Check for double push ability
                let (fwd2_sq, init_rank) = if color.is_white() {
                    (from_sq - 16, PAWN_INIT_WHITE_RANK)
                } else {
                    (from_sq + 16, PAWN_INIT_BLACK_RANK)
                };

                if init_rank.get_bit(from_sq) && ((!self.all_occupancies) & valid_mask).get_bit(fwd2_sq) {
                    move_list.insert(Move::new_custom(from_sq, fwd2_sq, Pawn, DoublePush))
                }
            }
        }
    }

    #[inline(always)]
    fn generate_pawn_captures<const HAS_ENPASSANT: bool>(&self, move_list: &mut MoveList, from_sq: u8, check_mask: Bitboard, pin_mask: Bitboard) {
        let color = self.active_color;
        let valid_mask = check_mask & pin_mask;

        let promotion_rank = if color.is_white() {
            PAWN_INIT_BLACK_RANK
        } else {
            PAWN_INIT_WHITE_RANK
        };

        let promoting = promotion_rank.get_bit(from_sq);

        let attacks = pawn_attacks(from_sq, color);

        let mut captures = attacks & valid_mask & self.color_bb(opposite_color(color));
        while let Some(sq) = captures.extract_bit() {
            if !promoting {
                move_list.insert(Move::new_normal(from_sq, sq, Pawn, true))
            } else {
                move_list.insert(Move::new_promotion(from_sq, sq, Queen,  true));
                move_list.insert(Move::new_promotion(from_sq, sq, Rook,   true));
                move_list.insert(Move::new_promotion(from_sq, sq, Bishop, true));
                move_list.insert(Move::new_promotion(from_sq, sq, Knight, true));
            }
        }

        // Return if no npassant
        if !HAS_ENPASSANT {
            return
        }

        let mut captures = attacks & pin_mask & self.enpassant_square;

        if let Some(enp_sq) = captures.extract_bit() {
            let captured = match color {
                White => enp_sq + 8,
                Black => enp_sq - 8,
            };

            // Check mask check
            if !(check_mask.get_bit(enp_sq) || check_mask.get_bit(captured)) {
                return
            }

            let pin_mask = self.generate_enpassant_pin_mask(color, from_sq);
            if !pin_mask.get_bit(captured) {
                // Not opening up after enpassant capture
                move_list.insert(Move::new_custom(from_sq, enp_sq, Pawn, EnpassantCapture))
            }
        }
    }

    #[inline(always)]
    fn generate_pawn_moves(&self, move_list: &mut MoveList, check_mask: Bitboard, hv_pin: Bitboard, d12_pin: Bitboard) {
        let color = self.active_color;
        let pawns = self.bb(color, Pawn);
        let has_enpassant = self.enpassant_square.is_not_empty();

        let mut hv_pinned_pawns = pawns & hv_pin;
        while let Some(sq) = hv_pinned_pawns.extract_bit() {
            self.generate_quiet_pawn_moves(move_list, sq, check_mask & hv_pin)
        }

        let mut d12_pinned_pawns = pawns & d12_pin;
        while let Some(sq) = d12_pinned_pawns.extract_bit() {
            generate_pawn_captures!(self, move_list, has_enpassant, sq, check_mask, d12_pin);
        }

        let mut unpinned_pawns = pawns & !(hv_pin | d12_pin);
        while let Some(sq) = unpinned_pawns.extract_bit() {
            self.generate_quiet_pawn_moves(move_list, sq, check_mask);
            generate_pawn_captures!(self, move_list, has_enpassant, sq, check_mask, Bitboard::FULL);
        }
    }

    #[inline(always)]
    fn add_normal_moves(&self, move_list: &mut MoveList, from_sq: u8, mut legal_to_sqs: Bitboard, piece: PieceType) {
        let color = self.active_color;
        while let Some(sq) = legal_to_sqs.extract_bit() {
            let is_capture = self.color_bb(opposite_color(color)).get_bit(sq);
            move_list.insert(Move::new_normal(from_sq, sq, piece, is_capture))
        }
    }

    #[inline(always)]
    fn generate_king_moves<const CHECK_CASTLING: bool>(&self, move_list: &mut MoveList) {
        let color = self.active_color;
        let attacked = 
            self.get_attacked_wo_king(color, Pawn) |
            self.get_attacked_wo_king(color, Knight) |
            self.get_attacked_wo_king(color, Bishop) |
            self.get_attacked_wo_king(color, Rook) |
            self.get_attacked_wo_king(color, Queen) |
            self.get_attacked_wo_king(color, King);

        let king_pos = self.king_position(color);
        let opp_or_empty = !self.color_bb(color);

        let legal = king_attacks(king_pos) & !attacked & opp_or_empty;

        self.add_normal_moves(move_list, king_pos, legal, King);

        if !CHECK_CASTLING || (attacked & self.bb(color, King)).is_not_empty() {
            return
        }

        // Castling
        match color {
            Color::White => {
                if self.castling_ability & (CastlingAbility::WhiteKingSide as u8) != 0 {
                    let none_attacked = (CastlingAbility::WhiteKingSide.attacked_mask() & attacked).is_empty();
                    let between_open =  (CastlingAbility::WhiteKingSide.open_mask() & self.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        move_list.insert(Move::new_custom(Square::e1 as u8, Square::g1 as u8, King, MoveType::CastleKingSide))
                    }
                }
                if self.castling_ability & (CastlingAbility::WhiteQueenSide as u8) != 0 {
                    let none_attacked = (CastlingAbility::WhiteQueenSide.attacked_mask() & attacked).is_empty();
                    let between_open =  (CastlingAbility::WhiteQueenSide.open_mask() & self.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        move_list.insert(Move::new_custom(Square::e1 as u8, Square::c1 as u8, King, MoveType::CastleQueenSide))
                    }
                }
            },
            Color::Black => {
                if self.castling_ability & (CastlingAbility::BlackKingSide as u8) != 0 {
                    let none_attacked = (CastlingAbility::BlackKingSide.attacked_mask() & attacked).is_empty();
                    let between_open =  (CastlingAbility::BlackKingSide.open_mask() & self.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        move_list.insert(Move::new_custom(Square::e8 as u8, Square::g8 as u8, King, MoveType::CastleKingSide))
                    }
                }
                if self.castling_ability & (CastlingAbility::BlackQueenSide as u8) != 0 {
                    let none_attacked = (CastlingAbility::BlackQueenSide.attacked_mask() & attacked).is_empty();
                    let between_open =  (CastlingAbility::BlackQueenSide.open_mask() &self.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        move_list.insert(Move::new_custom(Square::e8 as u8, Square::c8 as u8, King, MoveType::CastleQueenSide))
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn get_attacked_wo_king(&self, color: Color, piece_type: PieceType) -> Bitboard {
        let occ_wo_king = self.all_occupancies ^ self.bb(color, King);
        let opp_color = opposite_color(color);

        let mut bb = self.bb(opp_color, piece_type);
        let mut mask = Bitboard::EMPTY;
        while let Some(square) = bb.extract_bit() {
            mask |= get_attacks(square, opp_color, piece_type, occ_wo_king)
        };

        mask
    }

    #[inline(always)]
    pub fn generate_check_mask(&self, color: Color) -> Bitboard {
        let mut mask = Bitboard::EMPTY;
        let king_pos = self.king_position(color);
        let opp_color = opposite_color(color);

        let king_rays = hv_attacks(king_pos, self.all_occupancies) | d12_attacks(king_pos, self.all_occupancies);

        // Maybe move to pregenerated to optimize? TODO
        let mut hv_sliders = self.bb(opp_color, Rook) | self.bb(opp_color, Queen);
        while let Some(slider) = hv_sliders.extract_bit() {
            let slider_check_mask = SLIDER_HV_CHECK_MASK[king_pos as usize * 64 + slider as usize];

            if (slider_check_mask & king_rays) == slider_check_mask {
                //In check
                mask |= slider_check_mask;
            }
        }

        // Maybe move to pregenerated to optimize? TODO
        let mut d12_sliders = self.bb(opp_color, Bishop) | self.bb(opp_color, Queen);
        while let Some(slider) = d12_sliders.extract_bit() {
            let slider_check_mask = SLIDER_D12_CHECK_MASK[king_pos as usize * 64 + slider as usize];

            if (slider_check_mask & king_rays) == slider_check_mask {
                //In check
                mask |= slider_check_mask;
            }
        }

        mask |= pawn_attacks(king_pos, color) & self.bb(opp_color, Pawn);
        mask |= knight_attacks(king_pos) & self.bb(opp_color, Knight);

        if mask.is_not_empty() {
            mask
        } else {
            Bitboard::FULL
        }
    }

    #[inline(always)]
    pub fn generate_hv_pin_mask(&self, color: Color) -> Bitboard {
        let mut mask = 0;

        let opp_color = opposite_color(color);

        let mut h_sliders = RANK_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Rook) | self.bb(opp_color, Queen));
        while let Some(slider) = h_sliders.extract_bit() {
            mask |= pin_mask_h(self.all_occupancies, self.king_position(color), slider);
        }

        let mut v_sliders = FILE_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Rook) | self.bb(opp_color, Queen));
        while let Some(slider) = v_sliders.extract_bit() {
            mask |= pin_mask_v(self.all_occupancies, self.king_position(color), slider);
        }

        Bitboard(mask)
    }

    #[inline(always)]
    pub fn generate_d12_pin_mask(&self, color: Color) -> Bitboard {
        let mut mask = 0;

        let opp_color = opposite_color(color);

        let mut d1_sliders = D1_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Bishop) | self.bb(opp_color, Queen));
        while let Some(slider) = d1_sliders.extract_bit() {
            mask |= pin_mask_d1(self.all_occupancies, self.king_position(color), slider)
        }

        let mut d2_sliders = D2_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Bishop) | self.bb(opp_color, Queen));
        while let Some(slider) = d2_sliders.extract_bit() {
            mask |= pin_mask_d2(self.all_occupancies, self.king_position(color), slider)
        }
        
        Bitboard(mask)
    }

    #[inline(always)]
    pub fn generate_enpassant_pin_mask(&self, color: Color, from_sq: u8) -> Bitboard {
        let mut mask = 0;

        let opp_color = opposite_color(color);

        let occ = self.all_occupancies ^ 1 << from_sq;

        let mut h_sliders = RANK_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Rook) | self.bb(opp_color, Queen));
        while let Some(slider) = h_sliders.extract_bit() {
            mask |= pin_mask_h(occ, self.king_position(color), slider)
        }

        let mut d1_sliders = D1_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Bishop) | self.bb(opp_color, Queen));
        while let Some(slider) = d1_sliders.extract_bit() {
            mask |= pin_mask_d1(occ, self.king_position(color), slider)
        }

        let mut d2_sliders = D2_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Bishop) | self.bb(opp_color, Queen));
        while let Some(slider) = d2_sliders.extract_bit() {
            mask |= pin_mask_d2(occ, self.king_position(color), slider)
        }

        mask |= self.generate_d12_pin_mask(color);

        Bitboard(mask)
    }
}