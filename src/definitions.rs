

////////////////////////////////////////
/// General                          ///
////////////////////////////////////////

use std::fmt::Display;

use crate::{move_generator::MoveGenerator, position::Position};

use Piece::*;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum Square {
    a8,  b8,  c8,  d8,  e8,  f8,  g8,  h8,
    a7,  b7,  c7,  d7,  e7,  f7,  g7,  h7,
    a6,  b6,  c6,  d6,  e6,  f6,  g6,  h6,
    a5,  b5,  c5,  d5,  e5,  f5,  g5,  h5,
    a4,  b4,  c4,  d4,  e4,  f4,  g4,  h4,
    a3,  b3,  c3,  d3,  e3,  f3,  g3,  h3,
    a2,  b2,  c2,  d2,  e2,  f2,  g2,  h2,
    a1,  b1,  c1,  d1,  e1,  f1,  g1,  h1, 
    None
}

pub const SQUARE_STRINGS: [&str; 65] = [
    "a8",  "b8",  "c8",  "d8",  "e8",  "f8",  "g8",  "h8",
    "a7",  "b7",  "c7",  "d7",  "e7",  "f7",  "g7",  "h7",
    "a6",  "b6",  "c6",  "d6",  "e6",  "f6",  "g6",  "h6",
    "a5",  "b5",  "c5",  "d5",  "e5",  "f5",  "g5",  "h5",
    "a4",  "b4",  "c4",  "d4",  "e4",  "f4",  "g4",  "h4",
    "a3",  "b3",  "c3",  "d3",  "e3",  "f3",  "g3",  "h3",
    "a2",  "b2",  "c2",  "d2",  "e2",  "f2",  "g2",  "h2",
    "a1",  "b1",  "c1",  "d1",  "e1",  "f1",  "g1",  "h1", 
    "None"
];

pub const SQUARES: [Square; 65] = [
    Square::a8,  Square::b8,  Square::c8,  Square::d8,  Square::e8,  Square::f8,  Square::g8,  Square::h8,
    Square::a7,  Square::b7,  Square::c7,  Square::d7,  Square::e7,  Square::f7,  Square::g7,  Square::h7,
    Square::a6,  Square::b6,  Square::c6,  Square::d6,  Square::e6,  Square::f6,  Square::g6,  Square::h6,
    Square::a5,  Square::b5,  Square::c5,  Square::d5,  Square::e5,  Square::f5,  Square::g5,  Square::h5,
    Square::a4,  Square::b4,  Square::c4,  Square::d4,  Square::e4,  Square::f4,  Square::g4,  Square::h4,
    Square::a3,  Square::b3,  Square::c3,  Square::d3,  Square::e3,  Square::f3,  Square::g3,  Square::h3,
    Square::a2,  Square::b2,  Square::c2,  Square::d2,  Square::e2,  Square::f2,  Square::g2,  Square::h2,
    Square::a1,  Square::b1,  Square::c1,  Square::d1,  Square::e1,  Square::f1,  Square::g1,  Square::h1,
    Square::None
];

pub const PIECES: [Piece; 12] = [
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
];

pub fn square_from_string(string: &str) -> Square {
    let chars = string.as_bytes();
    let x = chars[0] - 97;
    let y = 8 - (chars[1] as char).to_digit(10).unwrap() as usize;
    SQUARES[8 * y + x as usize]
}

#[derive(Clone, Copy)]
pub enum CastlingAbility {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Color {
    White = 0,
    Black = 6,
}

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn is_slider(&self) -> bool {
        match self {
            Piece::WhiteBishop | Piece::WhiteRook | Piece::WhiteQueen |
            Piece::BlackBishop | Piece::BlackRook | Piece::BlackQueen  => true,
            _ => false
        }
    }

    pub fn piece_type(&self) -> PieceType {
        match self {
            WhitePawn =>   PieceType::Pawn,
            WhiteKnight => PieceType::Knight,
            WhiteBishop => PieceType::Bishop,
            WhiteRook =>   PieceType::Rook,
            WhiteQueen =>  PieceType::Queen,
            WhiteKing =>   PieceType::King,
            BlackPawn =>   PieceType::Pawn,
            BlackKnight => PieceType::Knight,
            BlackBishop => PieceType::Bishop,
            BlackRook =>   PieceType::Rook,
            BlackQueen =>  PieceType::Queen,
            BlackKing =>   PieceType::King,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PIECE_STRINGS[*self as usize])
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self::WhitePawn
    }
}

pub const PIECE_STRINGS: [&str; 13] = ["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k", "None"];

pub fn char_to_piece(char: char) -> Option<Piece> {
    match char {
        'P' => Some(Piece::WhitePawn),
        'R' => Some(Piece::WhiteRook),
        'N' => Some(Piece::WhiteKnight),
        'B' => Some(Piece::WhiteBishop),
        'Q' => Some(Piece::WhiteQueen),
        'K' => Some(Piece::WhiteKing),
        'p' => Some(Piece::BlackPawn),
        'r' => Some(Piece::BlackRook),
        'n' => Some(Piece::BlackKnight),
        'b' => Some(Piece::BlackBishop),
        'q' => Some(Piece::BlackQueen),
        'k' => Some(Piece::BlackKing),
        _ => None
    }
}

pub fn opposite_color(color: Color) -> Color {
    if color == Color::White { Color::Black } else { Color::White }
}

#[derive(PartialEq, Copy, Clone)]
pub enum MoveTypes {
    All,
    Quiescence,
}

impl Default for MoveTypes {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Copy, Clone)]
pub enum GenPhase {
    Interesting,
    Quiet,
    Done,
}

impl Default for GenPhase {
    fn default() -> Self {
        Self::Interesting
    }
}

pub struct Settings {
    pub threads: u8,
    pub transposition_table_mb: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self { threads: 1, transposition_table_mb: 128 }
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub from_sq: u8,
    pub to_sq: u8,
    pub piece: Piece,
    pub promotion: Piece,
    pub is_capture: bool,
    pub is_double_push: bool,
    pub is_enpassant: bool,
    pub is_castling: bool,

    pub score: u16,
}

impl Move {
    pub fn new(
        from_sq: u8,
        to_sq: u8,
        piece: Piece,
        promotion: Piece,
        is_capture: bool,
        is_double_push: bool,
        is_enpassant: bool,
        is_castling: bool) -> Self {
            Self {
                from_sq,
                to_sq,
                piece,
                promotion,
                is_capture,
                is_double_push,
                is_enpassant,
                is_castling,
                score: u16::MAX,
            }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {} -> {}", self.piece, SQUARE_STRINGS[self.from_sq as usize], SQUARE_STRINGS[self.to_sq as usize])
    }
}

impl Default for Move {
    fn default() -> Self {
        Self { 
            from_sq:        Default::default(),
            to_sq:          Default::default(),
            piece:          Default::default(),
            promotion:      Default::default(),
            is_capture:     Default::default(),
            is_double_push: Default::default(),
            is_enpassant:   Default::default(),
            is_castling:    Default::default(),
            score:          Default::default(),
        }
    }
}

/// A simple move generator that can be used as an iterator.\
/// Do not use for performance intensive tasks.
pub struct MoveIterator {
    generator: MoveGenerator,
    position: Position,
}

impl MoveIterator {
    pub fn new(position: Position, move_types: MoveTypes, sort: bool) -> Self {
        Self {
            generator: MoveGenerator::new(&position, move_types, sort),
            position
        }
    }
}

impl Iterator for MoveIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next_move(&self.position)
    }
}

pub struct SearchContext {
    // TranspositionTable
    // 
}

////////////////////////////////////////
/// Evaluation                       ///
////////////////////////////////////////

/// Mirror positional score tables for opposite side
pub const MIRRORED: [usize; 64] = 
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

pub const LOOKUP_RANK: [usize; 64] =
[
    7, 7, 7, 7, 7, 7, 7, 7,
    6, 6, 6, 6, 6, 6, 6, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    4, 4, 4, 4, 4, 4, 4, 4,
    3, 3, 3, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
	0, 0, 0, 0, 0, 0, 0, 0
];

pub const FILE_MASKS: [u64; 64] = generate_file_masks();
pub const RANK_MASKS: [u64; 64] = generate_rank_masks();
pub const ISOLATED_MASKS: [u64; 64] = generate_isolated_pawn_masks();

pub const WHITE_PASSED_PAWN_MASKS: [u64; 64] = generate_white_passed_pawn_masks();
pub const BLACK_PASSED_PAWN_MASKS: [u64; 64] = generate_black_passed_pawn_masks();

const fn generate_file_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;
            

            let mut i = 0;
            while i < 8 {
                mask |= (1 << f) << i*8;
                i += 1;
            }

            masks[r * 8 + f] = mask;

            f += 1;
        }
        r += 1;
    }

    masks
}

const fn generate_rank_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;
            
            let mut i = 0;
            while i < 8 {
                mask |= (1 << i) << 8*f;
                i += 1;
            }

            masks[r * 8 + f] = mask;

            f += 1;
        }
        r += 1;
    }

    masks
}

const fn generate_isolated_pawn_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;

            if f > 0 {
                mask |= FILE_MASKS[r*8+f - 1]
            }
            if f < 7 {
                mask |= FILE_MASKS[r*8+f + 1]
            }

            masks[r*8+f] = mask;
            
            f += 1;
        }
        r += 1;
    }

    masks
}

const fn generate_white_passed_pawn_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;

            mask |= FILE_MASKS[r*8+f];

            if f > 0 {
                mask |= FILE_MASKS[r*8+f - 1]
            }
            if f < 7 {
                mask |= FILE_MASKS[r*8+f + 1]
            }
            //For all ranks lower
            let mut rr = 7;
            while rr > r {
                mask ^= RANK_MASKS[rr*8] & mask;
                rr -= 1;
            }
            masks[r*8+f] = mask;
            
            f += 1;
        }
        r += 1;
    }

    masks
}

const fn generate_black_passed_pawn_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    let mut r = 0;
    while r < 8 {
        let mut f = 0;
        while f < 8 {
            let mut mask = 0;

            mask |= FILE_MASKS[r*8+f];

            if f > 0 {
                mask |= FILE_MASKS[r*8+f - 1]
            }
            if f < 7 {
                mask |= FILE_MASKS[r*8+f + 1]
            }
            //For all ranks lower
            let mut rr = 0;
            while rr < r {
                mask ^= RANK_MASKS[rr*8] & mask;
                rr += 1;
            }
            masks[r*8+f] = mask;
            
            f += 1;
        }
        r += 1;
    }

    masks
}