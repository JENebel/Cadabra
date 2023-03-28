use super::*;

use PieceType::*;
use MoveType::*;
use Color::*;
use CastlingSide::*;
use Square::*;
use bitintr::{Pext, Pdep};

macro_rules! generate_pawn_captures {
    ($pos: expr, $move_list: expr, $has_enpassant_sq: expr, $src: expr, $check_mask: expr, $pin_mask: expr) => {
        match $has_enpassant_sq {
            true =>  $pos.generate_pawn_captures::<true>($move_list, $src, $check_mask, $pin_mask),
            false => $pos.generate_pawn_captures::<false>($move_list, $src, $check_mask, $pin_mask)
        }
    };
}

impl Position {
    /// Generate all legal moves for the position
    #[inline(always)]
    pub fn generate_moves(&self) -> MoveList {
        if self.active_color.is_white() {
            self.generate_moves_internal::<true>()
        } else {
            self.generate_moves_internal::<false>()
        }
    }

    #[inline(always)]
    fn generate_moves_internal<const IS_WHITE: bool>(&self) -> MoveList {
        let mut move_list = MoveList::new();
        let color = if IS_WHITE { White } else { Black };

        let check_mask = self.generate_check_mask(color);

        // If in double check, only king can move
        let in_check = !(!check_mask).is_empty();
        let checkers = (check_mask & self.color_bb(color.opposite())).count_bits();
        if in_check && checkers > 1 {
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
        let  unpinned_knights = self.bb(color, Knight) & !(hv_pin | d12_pin);
        for src in unpinned_knights {
            let seen = knight_attacks(src);
            let legal = seen & opp_or_empty & check_mask;
            self.add_normal_moves(&mut move_list, src, legal, Knight)
        }

        // Rook moves
        let rooks = self.bb(color, Rook);
        let  pinned_rooks = rooks & hv_pin;
        for src in pinned_rooks {
            let seen = hv_attacks(src, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask & hv_pin;
            self.add_normal_moves(&mut move_list, src, legal, Rook)
        }

        let unpinned_rooks = rooks & !(hv_pin | d12_pin);
        for src in unpinned_rooks {
            let seen = hv_attacks(src, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask;
            self.add_normal_moves(&mut move_list, src, legal, Rook)
        }

        // Bishop moves
        let bishops = self.bb(color, Bishop);
        let pinned_bishops = bishops & d12_pin;
        for src in pinned_bishops {
            let seen = d12_attacks(src, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask & d12_pin;
            self.add_normal_moves(&mut move_list, src, legal, Bishop)
        }

        let unpinned_bishops = bishops & !(hv_pin | d12_pin);
        for src in unpinned_bishops {
            let seen = d12_attacks(src, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask;
            self.add_normal_moves(&mut move_list, src, legal, Bishop)
        }

        // Queen moves
        let queens = self.bb(color, Queen);
        let hv_pinned_queens = queens & hv_pin;
        for src in hv_pinned_queens {
            let seen = hv_attacks(src, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask & hv_pin;
            self.add_normal_moves(&mut move_list, src, legal, Queen)
        }

        let d12_pinned_queens = queens & d12_pin;
        for src in d12_pinned_queens {
            let seen = d12_attacks(src, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask & d12_pin;
            self.add_normal_moves(&mut move_list, src, legal, Queen)
        }

        let unpinned_queens = queens & !(hv_pin | d12_pin);
        for src in unpinned_queens {
            let seen = d12_attacks(src, self.all_occupancies) | hv_attacks(src, self.all_occupancies);
            let legal = seen & opp_or_empty & check_mask;
            self.add_normal_moves(&mut move_list, src, legal, Queen)
        }

        // King moves
        self.generate_king_moves::<true>(&mut move_list);

        move_list
    }

    #[inline(always)]
    fn add_normal_moves(&self, move_list: &mut MoveList, src: u8, legal_targets: Bitboard, piece: PieceType) {
        let color = self.active_color;
        for dst in legal_targets {
            let is_capture = self.color_bb(color.opposite()).get_bit(dst);
            move_list.push_move(Move::new_normal(src, dst, piece, is_capture));
        }
    }

    #[inline(always)]
    fn generate_quiet_pawn_moves(&self, move_list: &mut MoveList, src: u8, valid_mask: Bitboard) {
        let color = self.active_color;

        let dst = if color.is_white() {
            src - 8
        } else {
            src + 8
        };

        // If square in front free
        if !self.all_occupancies.get_bit(dst) {
            // Determine if promotion
            if END_RANKS.get_bit(dst) {
                if valid_mask.get_bit(dst) {
                    move_list.push_move(Move::new_promotion(src, dst, Queen,  false));
                    move_list.push_move(Move::new_promotion(src, dst, Rook,   false));
                    move_list.push_move(Move::new_promotion(src, dst, Bishop, false));
                    move_list.push_move(Move::new_promotion(src, dst, Knight, false));
                }
            }
            else {
                // Normal move
                if valid_mask.get_bit(dst) {
                    move_list.push_move(Move::new_normal(src, dst, Pawn, false));
                }

                // Check for double push ability
                let (double_push_sq, init_rank) = if color.is_white() {
                    (src - 16, PAWN_INIT_WHITE_RANK)
                } else {
                    (src + 16, PAWN_INIT_BLACK_RANK)
                };

                if init_rank.get_bit(src) && ((!self.all_occupancies) & valid_mask).get_bit(double_push_sq) {
                    move_list.push_move(Move::new(src, double_push_sq, Pawn, DoublePush));
                }
            }
        }
    }

    #[inline(always)]
    fn generate_pawn_captures<const HAS_ENPASSANT: bool>(&self, move_list: &mut MoveList, src: u8, check_mask: Bitboard, pin_mask: Bitboard) {
        let color = self.active_color;
        let valid_mask = check_mask & pin_mask;

        let promotion_rank = if color.is_white() {
            PAWN_INIT_BLACK_RANK
        } else {
            PAWN_INIT_WHITE_RANK
        };

        let promoting = promotion_rank.get_bit(src);

        let attacks = pawn_attacks(src, color);

        let captures = attacks & valid_mask & self.color_bb(color.opposite());
        for dst in captures {
            if !promoting {
                move_list.push_move(Move::new_normal(src, dst, Pawn, true));
            } else {
                move_list.push_move(Move::new_promotion(src, dst, Queen,  true));
                move_list.push_move(Move::new_promotion(src, dst, Rook,   true));
                move_list.push_move(Move::new_promotion(src, dst, Bishop, true));
                move_list.push_move(Move::new_promotion(src, dst, Knight, true));
            }
        }

        // Return if no npassant
        if !HAS_ENPASSANT {
            return
        }

        let captures = attacks & pin_mask & self.enpassant_square;

        for enp_sq in captures {
            let captured = match color {
                White => enp_sq + 8,
                Black => enp_sq - 8,
            };

            // Check mask check
            if !check_mask.get_bit(enp_sq) && !check_mask.get_bit(captured) {
                return
            }

            let pin_mask = self.generate_enpassant_pin_mask(color, src);
            if !pin_mask.get_bit(captured) {
                // Not opening up after enpassant capture
                move_list.push_move(Move::new(src, enp_sq, Pawn, EnpassantCapture));
            }
        }
    }

    #[inline(always)]
    fn generate_pawn_moves(&self, move_list: &mut MoveList, check_mask: Bitboard, hv_pin: Bitboard, d12_pin: Bitboard) {
        let color = self.active_color;
        let pawns = self.bb(color, Pawn);
        let has_enpassant = !self.enpassant_square.is_empty();

        let hv_pinned_pawns = pawns & hv_pin;
        for src in hv_pinned_pawns {
            self.generate_quiet_pawn_moves(move_list, src, check_mask & hv_pin)
        }

        let d12_pinned_pawns = pawns & d12_pin;
        for src in d12_pinned_pawns {
            generate_pawn_captures!(self, move_list, has_enpassant, src, check_mask, d12_pin);
        }

        let unpinned_pawns = pawns & !(hv_pin | d12_pin);
        for src in unpinned_pawns {
            self.generate_quiet_pawn_moves(move_list, src, check_mask);
            generate_pawn_captures!(self, move_list, has_enpassant, src, check_mask, Bitboard::FULL);
        }
    }

    #[inline(always)]
    fn generate_king_moves<const GEN_CASTLING: bool>(&self, move_list: &mut MoveList) {
        let color = self.active_color;
        let attacked = 
            self.get_attacked_wo_king(color, Pawn)   |
            self.get_attacked_wo_king(color, Knight) |
            self.get_attacked_wo_king(color, Bishop) |
            self.get_attacked_wo_king(color, Rook)   |
            self.get_attacked_wo_king(color, Queen)  |
            self.get_attacked_wo_king(color, King);

        let king_pos = self.king_position(color);
        let opp_or_empty = !self.color_bb(color);

        let legal = king_attacks(king_pos) & !attacked & opp_or_empty;

        self.add_normal_moves(move_list, king_pos, legal, King);

        if !GEN_CASTLING || !(attacked & self.bb(color, King)).is_empty() {
            return
        }

        // Castling
        match color {
            Color::White => {
                if self.castling_ability.is_side_available(WhiteKingSide) {
                    let none_attacked = (WhiteKingSide.attacked_mask() & attacked).is_empty();
                    let between_open =  (WhiteKingSide.open_mask() & self.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        move_list.push_move(Move::new(e1 as u8, g1 as u8, King, CastleKingSide));
                    }
                }
                if self.castling_ability.is_side_available(WhiteQueenSide) {
                    let none_attacked = (WhiteQueenSide.attacked_mask() & attacked).is_empty();
                    let between_open =  (WhiteQueenSide.open_mask() & self.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        move_list.push_move(Move::new(e1 as u8, c1 as u8, King, CastleQueenSide));
                    }
                }
            },
            Color::Black => {
                if self.castling_ability.is_side_available(BlackKingSide) {
                    let none_attacked = (BlackKingSide.attacked_mask() & attacked).is_empty();
                    let between_open =  (BlackKingSide.open_mask() & self.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        move_list.push_move(Move::new(e8 as u8, g8 as u8, King, CastleKingSide));
                    }
                }
                if self.castling_ability.is_side_available(BlackQueenSide) {
                    let none_attacked = (BlackQueenSide.attacked_mask() & attacked).is_empty();
                    let between_open =  (BlackQueenSide.open_mask() &self.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        move_list.push_move(Move::new(e8 as u8, c8 as u8, King, CastleQueenSide));
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn get_attacked_wo_king(&self, color: Color, piece_type: PieceType) -> Bitboard {
        let occ_wo_king = self.all_occupancies ^ self.bb(color, King);
        let opp_color = color.opposite();

        let bb = self.bb(opp_color, piece_type);
        let mut mask = Bitboard::EMPTY;
        for piece in bb {
            mask |= get_attacks(piece, opp_color, piece_type, occ_wo_king)
        };

        mask
    }

    #[inline(always)]
    fn generate_check_mask(&self, color: Color) -> Bitboard {
        let mut mask = Bitboard::EMPTY;
        let king_pos = self.king_position(color);
        let opp_color = color.opposite();

        let king_rays = hv_attacks(king_pos, self.all_occupancies) | d12_attacks(king_pos, self.all_occupancies);

        // Maybe move to pregenerated to optimize? TODO
        let hv_sliders = self.bb(opp_color, Rook) | self.bb(opp_color, Queen);
        for slider in hv_sliders {
            let slider_check_mask = SLIDER_HV_CHECK_MASK[king_pos as usize * 64 + slider as usize];

            if (slider_check_mask & king_rays) == slider_check_mask {
                //In check
                mask |= slider_check_mask;
            }
        }

        // Maybe move to pregenerated to optimize? TODO
        let d12_sliders = self.bb(opp_color, Bishop) | self.bb(opp_color, Queen);
        for slider in d12_sliders {
            let slider_check_mask = SLIDER_D12_CHECK_MASK[king_pos as usize * 64 + slider as usize];

            if (slider_check_mask & king_rays) == slider_check_mask {
                //In check
                mask |= slider_check_mask;
            }
        }

        mask |= pawn_attacks(king_pos, color) & self.bb(opp_color, Pawn);
        mask |= knight_attacks(king_pos) & self.bb(opp_color, Knight);

        if !mask.is_empty() {
            mask
        } else {
            Bitboard::FULL
        }
    }

    #[inline(always)]
    fn generate_hv_pin_mask(&self, color: Color) -> Bitboard {
        let mut mask = 0;

        let opp_color = color.opposite();

        let h_sliders = RANK_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Rook) | self.bb(opp_color, Queen));
        for slider in h_sliders {
            mask |= self.pin_mask_h(self.all_occupancies, slider);
        }

        let v_sliders = FILE_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Rook) | self.bb(opp_color, Queen));
        for slider in v_sliders {
            mask |= self.pin_mask_v(self.all_occupancies, slider);
        }

        Bitboard(mask)
    }

    #[inline(always)]
    fn generate_d12_pin_mask(&self, color: Color) -> Bitboard {
        let mut mask = 0;

        let opp_color = color.opposite();

        let d1_sliders = D1_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Bishop) | self.bb(opp_color, Queen));
        for slider in d1_sliders {
            mask |= self.pin_mask_d1(self.all_occupancies, slider)
        }

        let d2_sliders = D2_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Bishop) | self.bb(opp_color, Queen));
        for slider in d2_sliders {
            mask |= self.pin_mask_d2(self.all_occupancies, slider)
        }
        
        Bitboard(mask)
    }

    #[inline(always)]
    fn generate_enpassant_pin_mask(&self, color: Color, src: u8) -> Bitboard {
        let mut mask = 0;

        let opp_color = color.opposite();

        let occ = self.all_occupancies ^ 1 << src;

        let h_sliders = RANK_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Rook) | self.bb(opp_color, Queen));
        for slider in h_sliders {
            mask |= self.pin_mask_h(occ, slider)
        }

        let d1_sliders = D1_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Bishop) | self.bb(opp_color, Queen));
        for slider in d1_sliders {
            mask |= self.pin_mask_d1(occ, slider)
        }

        let d2_sliders = D2_MASKS[self.king_position(color) as usize] & (self.bb(opp_color, Bishop) | self.bb(opp_color, Queen));
        for slider in d2_sliders {
            mask |= self.pin_mask_d2(occ, slider)
        }

        mask |= self.generate_d12_pin_mask(color);

        Bitboard(mask)
    }

    #[inline(always)]
    fn pin_mask_h(&self, occ: Bitboard, slider_pos: u8) -> u64 {
        let king_pos = self.king_position(self.active_color) as usize;

        let rank = RANK_MASKS[king_pos];

        let pexed = occ.as_u64().pext(rank);

        let kp = LOOKUP_FILE[king_pos];
        let sq = LOOKUP_FILE[slider_pos as usize];

        let index = 2048*kp + 256*sq + pexed as usize;
        let mask = PIN_MASKS[index];

        mask.pdep(rank)
    }

    #[inline(always)]
    fn pin_mask_v(&self, occ: Bitboard, slider_pos: u8) -> u64 {
        let king_pos = self.king_position(self.active_color) as usize;

        let file = FILE_MASKS[king_pos];

        let pexed = occ.as_u64().pext(file);

        let kp = 7 - LOOKUP_RANK[king_pos];
        let sq = 7 - LOOKUP_RANK[slider_pos as usize];
        let index = 2048*kp + 256*sq + pexed as usize;

        let mask = PIN_MASKS[index];

        mask.pdep(file)
    }

    #[inline(always)]
    fn pin_mask_d1(&self, occ: Bitboard, slider_pos: u8) -> u64 {
        let king_pos = self.king_position(self.active_color) as usize;

        let diagonal = D1_MASKS[king_pos as usize];

        let pexed = occ.as_u64().pext(diagonal);
        let kp = LOOKUP_D2[king_pos as usize];
        let sq = LOOKUP_D2[slider_pos as usize];

        let index = 2048*kp + 256*sq + pexed as usize;
        let mask = PIN_MASKS[index];

        mask.pdep(diagonal)
    }

    #[inline(always)]
    fn pin_mask_d2(&self, occ: Bitboard, slider_pos: u8) -> u64 {
        let king_pos = self.king_position(self.active_color) as usize;

        let diagonal = D2_MASKS[king_pos as usize];

        let pexed = occ.as_u64().pext(diagonal);

        // sq and kp are flipped to get correct mask
        let kp = LOOKUP_D1[king_pos as usize];
        let sq = LOOKUP_D1[slider_pos as usize];

        let index = 2048*kp + 256*sq + pexed as usize;
        let mask = PIN_MASKS[index];

        mask.pdep(diagonal)
    }
}