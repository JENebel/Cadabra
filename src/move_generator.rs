use crate::{position::Position, definitions::*, bitboard::Bitboard, attack_tables::*};
use PieceType::*;

// Macros to expand const generics for move generation
macro_rules! generate_moves_match_has_enpassant {
    ($move_gen: expr, $pos: expr, $is_white: expr, $phase: expr, $is_sorting: expr) => {
        match $pos.enpassant_square.is_some() {
            true =>  $move_gen.generate_moves::<$is_white, $phase, $is_sorting, true>($pos),
            false => $move_gen.generate_moves::<$is_white, $phase, $is_sorting, false>($pos),
        }
    };
}
macro_rules! generate_moves_match_is_sorting {
    ($move_gen: expr, $pos: expr, $is_white: expr, $phase: expr) => {
        match $move_gen.is_sorting {
            true =>  generate_moves_match_has_enpassant!($move_gen, $pos, $is_white, $phase, true),
            false => generate_moves_match_has_enpassant!($move_gen, $pos, $is_white, $phase, true),
        }
    };
}
macro_rules! generate_moves_match_phase {
    ($move_gen: expr, $pos: expr, $is_white: expr) => {
        match $move_gen.phase {
            GenPhase::Interesting => generate_moves_match_is_sorting!($move_gen, $pos, $is_white, 0),
            GenPhase::Quiet =>       generate_moves_match_is_sorting!($move_gen, $pos, $is_white, 1),
            GenPhase::Done =>        return None
        }
    };
}
macro_rules! generate_moves {
    ($move_gen: expr, $pos: expr) => {
        // Match color
        match $pos.active_color {
            Color::White => generate_moves_match_phase!($move_gen, $pos, true),
            Color::Black => generate_moves_match_phase!($move_gen, $pos, false),
        }
    };
}

pub struct MoveGenerator {
    move_types: MoveTypes,
    phase: GenPhase,
    is_sorting: bool,

    insert_index: usize,
    extract_index: usize,

    move_list: [Move; 100],
}

impl MoveGenerator {
    /// Creates a new move generator
    pub fn new(position: &Position, move_types: MoveTypes, sort: bool) -> Self {
        Self {
            move_types,
            phase: Default::default(),
            is_sorting: sort,
            
            insert_index: 0,
            extract_index: 0,

            move_list: [Default::default(); 100], // Check if this is necessary
        }
    }

    pub fn add_pv_move(&mut self, pv_move: Move) {
        self.insert(pv_move)
    }

    /// Gets the next move in the position
    pub fn next_move(&mut self, pos: &Position) -> Option<Move> {
        // Try generating more moves until some are found, or there are none left
        while self.extract_index == self.insert_index {
            generate_moves!(self, pos)
        }

        if self.is_sorting {
            Some(self.extract_best())
        } else {
            Some(self.extract_first())
        }
    }

    /// Extracts the best move in the list
    fn extract_best(&mut self) -> Move {
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
        extracted
    }

    /// Extract the first item in the list
    fn extract_first(&mut self) -> Move {
        let extracted = self.move_list[self.extract_index];
        self.extract_index += 1;
        extracted
    }

    #[inline(always)]
    fn insert_and_score(&mut self, new_move: &mut Move, is_sorting: bool) {
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
    fn generate_moves<const IS_WHITE: bool, const PHASE: u8, const IS_SORTING: bool, const HAS_ENPASSANT: bool>(&mut self, pos: &Position) {
        let active_color = if IS_WHITE { Color::White } else { Color::Black };

        let gen_phase: GenPhase = unsafe {
            std::mem::transmute(PHASE) // Always a legal phase!
        };

        let is_sorting = IS_SORTING;
        let has_enpassant = HAS_ENPASSANT;


        let check_mask = Self::get_check_mask(pos, active_color);

        // Go straight to check evasions if in check
        {
            let checkers = check_mask.and(pos.get_color_bitboard(opposite_color(active_color))).count();
            if !check_mask.not().is_empty() && checkers > 0 {
                if checkers > 1 {
                    // Double check, only king can move
                    self.generate_king_moves(pos, check_mask)
                } else {
                    self.generate_check_evasions(pos, check_mask);
                }
                return
            }
        }

        match active_color {
            Color::White => { },
            Color::Black => { },
        }

        match gen_phase {
            GenPhase::Interesting => {
                if self.move_types == MoveTypes::All {
                    self.phase = GenPhase::Quiet
                } else {
                    self.phase = GenPhase::Done
                }
            },
            GenPhase::Quiet => {
                self.phase = GenPhase::Done
            },
            _ => { }
        }
    }

    #[inline(always)]
    pub fn get_pin_mask(pos: &mut Position, color: Color, square: u8, piece_type: PieceType) -> Bitboard {
        pos.remove_piece(color, piece_type, square);

        let mask = MoveGenerator::get_check_mask(pos, color);

        pos.place_piece(color, piece_type, square);

        mask
    }

    /// Should be delegated to pregenerated Constants for sliders
    #[inline(always)]
    fn get_check_mask(pos: &Position, active_color: Color) -> Bitboard {
        let mut mask = Bitboard::new_blank();

        let opp_color = opposite_color(active_color);

        let king_pos = pos.king_position(active_color);

        // Leapers
        mask = mask.or(
            (get_pawn_attack_table(king_pos, opp_color).and(pos.get_bitboard(opp_color, Pawn))).or(
            get_knight_attack_table(king_pos).and(pos.get_bitboard(opp_color, Knight)))
        );

        // Hv Sliders
        {
            let opp_hv_sliders = pos.get_bitboard(opp_color, Rook).or(
                pos.get_bitboard(opp_color, Queen)
            );

            let king_file = Bitboard::from(FILE_MASKS[king_pos as usize]);
            let king_rank = Bitboard::from(RANK_MASKS[king_pos as usize]);

            let king_hv_rays = get_rook_attack_table(king_pos, pos.all_occupancies);

            let mut sliders = opp_hv_sliders;
            while let Some(slider) = sliders.extract_bit() {
                let mut slider_board = Bitboard::new_blank();
                slider_board.set_bit(slider);
                let slider_rays = (get_rook_attack_table(slider, pos.all_occupancies).and(pos.all_occupancies.not())).or(slider_board);

                let slider_hori = slider_rays.and(Bitboard::from(RANK_MASKS[slider as usize]));
                let king_hori = king_hv_rays.and(king_rank);
                mask = mask.or(king_hori.and(slider_hori));

                let slider_vert = slider_rays.and(Bitboard::from(FILE_MASKS[slider as usize]));
                let king_vert = king_hv_rays.and(king_file);
                mask = mask.or(king_vert.and(slider_vert));
            }
        }
        
        // Diagonal Sliders
        {
            let opp_diag_sliders = pos.get_bitboard(opp_color, Bishop).or(
                pos.get_bitboard(opp_color, Queen)
            );

            let king_diag1 = Bitboard::from(DIAG1_MASKS[king_pos as usize]);
            let king_diag2 = Bitboard::from(DIAG2_MASKS[king_pos as usize]);

            let king_diag_rays = get_bishop_attack_table(king_pos, pos.all_occupancies);

            let mut sliders = opp_diag_sliders;
            while let Some(slider) = sliders.extract_bit() {
                let mut slider_board = Bitboard::new_blank();
                slider_board.set_bit(slider);
                let slider_rays = (get_bishop_attack_table(slider, pos.all_occupancies)).and(pos.all_occupancies.not()).or(slider_board);

                let slider_hori = slider_rays.and(Bitboard::from(DIAG1_MASKS[slider as usize]));
                let king_hori = king_diag_rays.and(king_diag1);
                mask = mask.or(king_hori.and(slider_hori));

                let slider_vert = slider_rays.and(Bitboard::from(DIAG2_MASKS[slider as usize]));
                let king_vert = king_diag_rays.and(king_diag2);
                mask = mask.or(king_vert.and(slider_vert));
            }
        }

        if mask.is_empty() {
            Bitboard::new_full()
        } else {
            mask
        }
    }

    /// Generate check evasions
    #[inline(always)]
    fn generate_check_evasions(&mut self, pos: &Position, check_mask: Bitboard) {
        ///////
        
        todo!();

        // Should not generate more moves as this generates all possible
        self.phase = GenPhase::Done
    }

    /// Generates all legal king moves
    #[inline(always)]
    fn generate_king_moves(&mut self, pos: &Position, check_mask: Bitboard) {
        
    }
}

#[test]
pub fn test() {
    let mut pos = Position::new_from_fen("8/8/8/4K2B/8/8/4p1R1/3k4 b - - 0 1").unwrap();

    println!("{}", MoveGenerator::get_pin_mask(&mut pos, Color::Black, square_from_string("e2") as u8, PieceType::Pawn));

    /*pos = Position::new_from_fen("8/8/8/4K3/8/8/2k3R1/8 b - - 0 1").unwrap();

    println!("{}", MoveGenerator::get_check_mask(&pos, Color::Black));*/
}