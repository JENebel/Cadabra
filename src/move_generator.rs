use super::*;

use PieceType::*;
use MoveType::*;

// Macros to expand const generics for move generation
macro_rules! generate_moves_match_has_enpassant {
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
}

pub struct MoveList {
    insert_index: usize,
    extract_index: usize,

    move_list: [Move; 100],
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

impl MoveList {
    /// Creates a new move list and populates it with all legal moves in the position
    pub fn new(position: &Position, is_quiescence: bool, sort: bool, pv_move: Option<Move>) -> Self {
        let mut list = Self {
            insert_index: 0,
            extract_index: 0,

            move_list: [Default::default(); 100], // Check if this is necessary
        };

        if let Some(pv) = pv_move {
            list.insert(pv);
        }

        generate_moves!(list, position, is_quiescence, sort);

        list
    }

    pub fn length(&self) -> usize {
        self.insert_index
    }

    /// Extracts the best move in the list
    #[inline(always)]
    pub fn extract_best(&mut self) -> Option<Move> {
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

    /// Inserts the move into the list, and scores it if 
    #[inline(always)]
    fn insert_and_score(&mut self, new_move: &mut Move, is_sorting: bool, is_quiescence: bool) {
        if is_quiescence && !new_move.is_capture() {
            return
        }

        if is_sorting {
            Self::score_move(new_move) // Maybe handle scoring different
        }

        self.insert(*new_move)
    }

    #[inline(always)]
    fn insert(&mut self, new_move: Move) {
        self.move_list[self.insert_index] = new_move;
        self.insert_index += 1;
    }

    fn score_move(m: &mut Move) {
        m.score = 10; // Fake it. Should probably be moved to seperate place
    }

    /// Generate more legal moves for the position
    #[inline(always)]
    fn generate_moves<const IS_QUIESCENCE: bool, const IS_WHITE: bool, const IS_SORTING: bool, const HAS_ENPASSANT: bool>(&mut self, pos: &Position) {
        let active_color = if IS_WHITE { Color::White } else { Color::Black };

        let is_sorting = IS_SORTING;
        let has_enpassant = HAS_ENPASSANT;
        let is_quiescence = IS_QUIESCENCE;

        let opp_color = opposite_color(active_color);

        let attacked_squares = {
            pos.get_attacked(opp_color, Pawn) |
            pos.get_attacked(opp_color, Knight) |
            pos.get_attacked(opp_color, Rook) |
            pos.get_attacked(opp_color, Bishop) |
            pos.get_attacked(opp_color, Queen) |
            pos.get_attacked(opp_color, King)
        };

        // Go straight to check evasions if in check
        {
            let check_mask = Self::get_check_mask(pos, active_color, true);
            let checkers = (check_mask & pos.color_bb(opposite_color(active_color))).count();
            if !(!check_mask.is_empty()) && checkers > 0 {
                if checkers == 1 {
                    let valid_square_and_checkmask = (!pos.color_bb(active_color)) & check_mask;
                    // Single check, generate evasions
                    self.generate_pawn_moves(pos, active_color, is_sorting, false, has_enpassant, valid_square_and_checkmask);
                    self.generate_normal_moves(pos, active_color, is_sorting, Knight, valid_square_and_checkmask, false);
                    self.generate_normal_moves(pos, active_color, is_sorting, Bishop, valid_square_and_checkmask, false);
                    self.generate_normal_moves(pos, active_color, is_sorting, Rook, valid_square_and_checkmask, false);
                    self.generate_normal_moves(pos, active_color, is_sorting, Queen, valid_square_and_checkmask, false);
                }
                // Double check => only the king can move
                self.generate_king_moves(pos, active_color, attacked_squares, is_sorting, is_quiescence);
                return
            }
        }

        let empty_or_enemy = !pos.color_bb(active_color);

        // Castling
        match active_color {
            Color::White => {
                if pos.castling_ability & (CastlingAbility::WhiteKingSide as u8) != 0 {
                    let none_attacked = (CastlingAbility::WhiteKingSide.attacked_mask() & attacked_squares).is_empty();
                    let between_open =  (CastlingAbility::WhiteKingSide.open_mask() & pos.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        self.insert_and_score(&mut Move::new_custom(Square::e1 as u8, Square::g1 as u8, King, MoveType::CastleKingSide), is_sorting, is_quiescence)
                    }
                }
                if pos.castling_ability & (CastlingAbility::WhiteQueenSide as u8) != 0 {
                    let none_attacked = (CastlingAbility::WhiteQueenSide.attacked_mask() & attacked_squares).is_empty();
                    let between_open =  (CastlingAbility::WhiteQueenSide.open_mask() & pos.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        self.insert_and_score(&mut Move::new_custom(Square::e1 as u8, Square::c1 as u8, King, MoveType::CastleQueenSide), is_sorting, is_quiescence)
                    }
                }
            },
            Color::Black => {
                if pos.castling_ability & (CastlingAbility::BlackKingSide as u8) != 0 {
                    let none_attacked = (CastlingAbility::BlackKingSide.attacked_mask() & attacked_squares).is_empty();
                    let between_open =  (CastlingAbility::BlackKingSide.open_mask() & pos.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        self.insert_and_score(&mut Move::new_custom(Square::e8 as u8, Square::g8 as u8, King, MoveType::CastleKingSide), is_sorting, is_quiescence)
                    }
                }
                if pos.castling_ability & (CastlingAbility::BlackQueenSide as u8) != 0 {
                    let none_attacked = (CastlingAbility::BlackQueenSide.attacked_mask() & attacked_squares).is_empty();
                    let between_open =  (CastlingAbility::BlackQueenSide.open_mask() &pos.all_occupancies).is_empty();
                    if none_attacked && between_open {
                        self.insert_and_score(&mut Move::new_custom(Square::e8 as u8, Square::c8 as u8, King, MoveType::CastleQueenSide), is_sorting, is_quiescence)
                    }
                }
            }
        }
        
        self.generate_pawn_moves(pos, active_color, is_sorting, is_quiescence, has_enpassant, Bitboard::FULL);
        self.generate_normal_moves(pos, active_color, is_sorting, Knight, empty_or_enemy, is_quiescence);
        self.generate_normal_moves(pos, active_color, is_sorting, Bishop, empty_or_enemy, is_quiescence);
        self.generate_normal_moves(pos, active_color, is_sorting, Rook, empty_or_enemy, is_quiescence);
        self.generate_normal_moves(pos, active_color, is_sorting, Queen, empty_or_enemy, is_quiescence);
        self.generate_king_moves(pos, active_color, attacked_squares, is_sorting, is_quiescence);
    }

    #[inline(always)]
    fn generate_pawn_moves(&mut self, pos: &Position, color: Color, is_sorting: bool, is_quiescence: bool, has_enpassant: bool, valid_mask: Bitboard) {
        let mut pawns = pos.bb(color, Pawn);
        while let Some(from_square) = pawns.extract_bit() {
            // Quiet target square
            let to_square = match color {
                Color::White => (from_square as i8 - 8) as u8,
                Color::Black => from_square + 8,
            };

            let raw_captures = pawn_attacks(from_square, color);

            // Enpassant
            if has_enpassant {
                let enp_square = unsafe { pos.enpassant_square.unwrap_unchecked() };
                if (raw_captures & valid_mask).get_bit(enp_square as u8) {
                    let captured_square = match color {
                        Color::White => enp_square as u8 + 8,
                        Color::Black => enp_square as u8 - 8,
                    };
                    if Self::enpassant_pin_mask(pos, color, captured_square, from_square as u8).get_bit(enp_square as u8) {
                        self.insert_and_score(&mut Move::new_custom(from_square, enp_square as u8, Pawn, EnpassantCapture), is_sorting, false)
                    }
                }
            }

            let valid_mask = valid_mask & Self::get_pin_mask(pos, color, from_square, Pawn);

            let mut captures = raw_captures & pos.color_bb(opposite_color(color)) & valid_mask;

            // Promotions
            if END_RANKS_MASK.get_bit(to_square) {
                // Regular promotion
                if !pos.all_occupancies.get_bit(to_square) && valid_mask.get_bit(to_square){
                    self.insert_and_score(&mut Move::new_promotion(from_square, to_square, Queen,  false), is_sorting, false);
                    self.insert_and_score(&mut Move::new_promotion(from_square, to_square, Rook,   false), is_sorting, false);
                    self.insert_and_score(&mut Move::new_promotion(from_square, to_square, Bishop, false), is_sorting, false);
                    self.insert_and_score(&mut Move::new_promotion(from_square, to_square, Knight, false), is_sorting, false);
                }

                // Capture promotions
                while let Some(to_square) = captures.extract_bit() {
                    self.insert_and_score(&mut Move::new_promotion(from_square, to_square, Queen,  true), is_sorting, false);
                    self.insert_and_score(&mut Move::new_promotion(from_square, to_square, Rook,   true), is_sorting, false);
                    self.insert_and_score(&mut Move::new_promotion(from_square, to_square, Bishop, true), is_sorting, false);
                    self.insert_and_score(&mut Move::new_promotion(from_square, to_square, Knight, true), is_sorting, false);
                }

                return
            }

            // Regular captures
            while let Some(to_square) = captures.extract_bit() {
                self.insert_and_score(&mut Move::new_normal(from_square, to_square, Pawn, true), is_sorting, false)
            }

            // Quiet & double push
            if !is_quiescence && !pos.all_occupancies.get_bit(to_square) {
                // Normal move
                if valid_mask.get_bit(to_square) {
                    self.insert_and_score(&mut Move::new_normal(from_square, to_square, Pawn, false), is_sorting, false);
                }

                // Double push
                // Only possible if pawn hasn't moved. Needs to come after promotions to not override them
                if PAWN_INIT_RANKS_MASK.get_bit(from_square) {
                    let double_push_square = match color {
                        Color::White => (from_square as i8 - 16) as u8,
                        Color::Black => from_square + 16,
                    };

                    if !pos.all_occupancies.get_bit(double_push_square) && valid_mask.get_bit(double_push_square) {
                        self.insert_and_score(&mut Move::new_custom(from_square, double_push_square, Pawn, DoublePush), is_sorting, false);
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn generate_normal_moves(&mut self, pos: &Position, color: Color, is_sorting: bool, piece_type: PieceType, empty_or_enemy: Bitboard, is_quiescence: bool) {
        let mut pieces = pos.bb(color, piece_type);
        while let Some(from_square) = pieces.extract_bit() {
            let mut moves = get_attacks(from_square, color, piece_type, pos.all_occupancies) & empty_or_enemy & Self::get_pin_mask(pos, color, from_square, piece_type);
        
            while let Some(to_square) = moves.extract_bit() {
                let is_capture = pos.all_occupancies.get_bit(to_square);
                self.insert_and_score(&mut Move::new_normal(from_square, to_square, piece_type, is_capture), is_sorting, is_quiescence)
            }
        }
    }

    /// Generates all legal king moves
    #[inline(always)]
    fn generate_king_moves(&mut self, pos: &Position, color: Color, attacked_squares: Bitboard, is_sorting: bool, is_quiescence: bool) {
        let king_pos = pos.king_position(color);

        let opp_color = opposite_color(color);

        let mut pos = *pos;
        pos.remove_piece(color, King, king_pos);

        let attacked_squares = attacked_squares |
            pos.get_attacked(opp_color, Bishop) |
            pos.get_attacked(opp_color, Queen) |
            pos.get_attacked(opp_color, King);

        let mut legal_moves = king_attacks(king_pos) & !attacked_squares & (!pos.color_bb(color));
        
        while let Some(to_square) = legal_moves.extract_bit() {
            let is_capture = pos.all_occupancies.get_bit(to_square);
            self.insert_and_score(&mut Move::new_normal(king_pos, to_square, King, is_capture), is_sorting, is_quiescence)
        }
    }

    /// Should be delegated to pregenerated Constants for sliders
    #[inline(always)]
    pub fn get_pin_mask(pos: &Position, color: Color, square: u8, piece_type: PieceType) -> Bitboard {
        let mut pos = *pos;

        pos.remove_piece(color, piece_type, square);

        MoveList::get_check_mask(&pos, color, false)
    }

    #[inline(always)]
    /// Call with the color of the active player
    pub fn enpassant_pin_mask(pos: &Position, color: Color, captured: u8, square: u8) -> Bitboard {
        let mut pos = *pos;

        pos.remove_piece(opposite_color(color), Pawn, captured);

        Self::get_pin_mask(&pos, color, square, Pawn)
    }

    /// Should be delegated to pregenerated Constants for sliders
    #[inline(always)]
    fn get_check_mask(pos: &Position, color: Color, include_leapers: bool) -> Bitboard {
        let mut mask = Bitboard::EMPTY;

        let opp_color = opposite_color(color);

        let king_pos = pos.king_position(color);

        // Leapers
        if include_leapers {
            mask |=
                (pawn_attacks(king_pos, color) & pos.bb(opp_color, Pawn)) | (
                knight_attacks(king_pos) & pos.bb(opp_color, Knight));
        }

        // Hv Sliders
        {
            let opp_hv_sliders = pos.bb(opp_color, Rook) |
                pos.bb(opp_color, Queen);

            let king_file = FILE_MASKS[king_pos as usize];
            let king_rank = RANK_MASKS[king_pos as usize];

            let king_hv_rays = rook_attacks(king_pos, pos.all_occupancies);

            let mut sliders = opp_hv_sliders;
            while let Some(slider) = sliders.extract_bit() {
                let mut slider_board = Bitboard::EMPTY;
                slider_board.set_bit(slider);
                let slider_rays = rook_attacks(slider, pos.all_occupancies) | !pos.all_occupancies;

                let slider_hori = slider_rays | RANK_MASKS[slider as usize] | slider_board;
                let king_hori = king_hv_rays & king_rank;
                mask |= king_hori & slider_hori;

                let slider_vert = slider_rays & FILE_MASKS[slider as usize] | slider_board;
                let king_vert = king_hv_rays & king_file;
                mask |= king_vert & slider_vert;
            }
        }
        
        // Diagonal Sliders
        {
            let opp_diag_sliders = pos.bb(opp_color, Bishop) | pos.bb(opp_color, Queen);

            let king_diag_rays = bishop_attacks(king_pos, pos.all_occupancies);

            let mut sliders = opp_diag_sliders;
            while let Some(slider) = sliders.extract_bit() {
                let mut slider_board = Bitboard::EMPTY;
                slider_board.set_bit(slider);
                let slider_rays = bishop_attacks(slider, pos.all_occupancies) & !pos.all_occupancies;

                let slider_diag1 = slider_rays & DIAG1_MASKS[slider as usize] | slider_board;
                let king_diag1_ray = king_diag_rays & DIAG1_MASKS[king_pos as usize];
                mask |= king_diag1_ray & slider_diag1;

                let slider_diag2 = slider_rays & DIAG2_MASKS[slider as usize] | slider_board;
                let king_diag2_ray = king_diag_rays & DIAG2_MASKS[king_pos as usize];
                mask |= king_diag2_ray & slider_diag2;
            }
        }

        if mask.is_empty() {
            Bitboard::FULL
        } else {
            mask
        }
    }
}

#[test]
pub fn test() {
    let mut pos = Position::new_from_start_pos();

    // Double pawn push block
    // let mut pos = Position::new_from_fen("rnb1kb1r/pp1p2pp/2p5/q7/8/3P4/PPP1PPPP/RN2KBNR w KQkq - 0 1").unwrap();

    // Illegal enpassant pin
    // let mut pos = Position::new_from_fen("1k6/8/8/1q1pP1K1/8/8/8/8 w - d6 0 1").unwrap();

    // Legal enpassant pin
    // let mut pos = Position::new_from_fen("1q6/8/8/3pP3/8/8/7K/8 w - d5 0 1").unwrap();

    pos.pretty_print();

    let moves = MoveList::new(&mut pos, false, false, None).collect::<Vec<Move>>();

    println!("Moves: {}", moves.len());
    println!("Captures: {}", moves.iter().filter(|m| m.is_capture()).count());
    println!("E.p.: {}", moves.iter().filter(|m| if let EnpassantCapture = m.move_type { true } else { false }).count());
    println!("Castles: {}", moves.iter().filter(|m| m.move_type == CastleKingSide || m.move_type == CastleQueenSide).count());
    println!("Promotions: {}", moves.iter().filter(|m| if let Promotion(_) = m.move_type { true } else { false } || if let CapturePromotion(_) = m.move_type { true } else { false }).count());

    println!();
    
    for m in moves {
        println!("{}", m)
    }
}