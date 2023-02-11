use super::*;

use PieceType::*;
use MoveType::*;

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
    /// Creates a new move list and populates it with all legal moves in the position
    pub fn generate_from(position: &Position, is_quiescence: bool, sort: bool, pv_move: Option<Move>) -> Self {
        let mut list = Self {
            insert_index: 0,
            extract_index: 0,

            move_list: [Default::default(); 100], // Check if this is necessary
        };

        if let Some(pv) = pv_move {
            list.insert(pv);
        }

        //generate_moves!(list, position, is_quiescence, sort);

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

        println!("{new_move}");

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
    fn generate_moves() {

    }

    #[inline(always)]
    fn generate_hv_pin_mask(pos: &Position, color: Color) -> u64 {
        let mut mask = 0;

        let opp_color = opposite_color(color);

        let mut hv_sliders = pos.bb(opp_color, Rook) | pos.bb(color, Queen);
        while let Some(slider) = hv_sliders.extract_bit() {
            mask |= pin_mask_hv(pos.all_occupancies, pos.king_position(color), slider)
        }

        mask
    }

    #[inline(always)]
    fn generate_d12_pin_mask(pos: &Position, color: Color) -> u64 {
        let mut mask = 0;

        let opp_color = opposite_color(color);

        let mut d12_sliders = pos.bb(opp_color, Bishop) | pos.bb(color, Queen);
        while let Some(slider) = d12_sliders.extract_bit() {
            mask |= pin_mask_hv(pos.all_occupancies, pos.king_position(color), slider)
        }

        mask
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