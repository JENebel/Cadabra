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
    pub fn generate_moves_internal(&self) -> MoveList {
        let mut move_list = MoveList::new();

        let color = self.active_color;
        let opp_color = opposite_color(self.active_color);

        let check_mask = self.generate_check_mask(color);

        // If in double check, only king can move
        let checkers = (check_mask & self.color_bb(opposite_color(color))).count();
        if !(!check_mask.is_empty()) && checkers > 0 {
            self.generate_king_moves(&mut move_list);
            return move_list
        }

        // Generate the pin masks
        let hv_pin = self.generate_hv_pin_mask(color);
        let d12_pin = self.generate_d12_pin_mask(color);
        let opp_or_empty = !self.color_bb(color);
        
        // Pawn moves
        self.generate_pawn_moves(&mut move_list);

        // Knight moves. Only unpinned can move
        let mut unpinned_knights = self.bb(color, Knight);
        while let Some(sq) = unpinned_knights.extract_bit() {
            let seen = knight_attacks(sq);
            let legal = seen & opp_or_empty & check_mask & !(hv_pin | d12_pin);
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
        self.generate_king_moves(&mut move_list);

        move_list
    }

    #[inline(always)]
    fn generate_pawn_moves(&self, move_list: &mut MoveList) {

    }

    #[inline(always)]
    fn add_normal_moves(&self, move_list: &mut MoveList, from_sq: u8, mut legal_to_sq: Bitboard, piece: PieceType) {
        let color = self.active_color;
        while let Some(sq) = legal_to_sq.extract_bit() {
            let is_capture = self.color_bb(opposite_color(color)).get_bit(sq);
            move_list.insert(Move::new_normal(from_sq, sq, piece, is_capture))
        }
    }

    #[inline(always)]
    fn generate_king_moves(&self, move_list: &mut MoveList) {
        let color = self.active_color;
        let attacked = 
            self.get_attacked_wo_king(color, Pawn) |
            self.get_attacked_wo_king(color, Knight) |
            self.get_attacked_wo_king(color, Bishop) |
            self.get_attacked_wo_king(color, Rook) |
            self.get_attacked_wo_king(color, Queen);

        let king_pos = self.king_position(color);
        let opp_or_empty = !self.color_bb(color);

        let legal = king_attacks(king_pos) & !attacked & opp_or_empty;

        self.add_normal_moves(
            move_list, 
            self.king_position(color), 
            legal, 
            King
        );
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

        let mut hv_sliders = HV_RAYS[self.king_position(color) as usize] & (self.bb(opp_color, Rook) | self.bb(opp_color, Queen));
        while let Some(slider) = hv_sliders.extract_bit() {
            mask |= pin_mask_hv(self.all_occupancies, self.king_position(color), slider)
        }

        Bitboard(mask)
    }

    #[inline(always)]
    pub fn generate_d12_pin_mask(&self, color: Color) -> Bitboard {
        let mut mask = 0;

        let opp_color = opposite_color(color);

        let mut d12_sliders = D12_RAYS[self.king_position(color) as usize] & (self.bb(opp_color, Bishop) | self.bb(opp_color, Queen));
        while let Some(slider) = d12_sliders.extract_bit() {
            mask |= pin_mask_d12(self.all_occupancies, self.king_position(color), slider)
        }

        Bitboard(mask)
    }
}

#[test]
pub fn test() {
    
}