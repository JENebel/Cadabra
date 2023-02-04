include!(concat!(env!("OUT_DIR"), "/consts.rs"));

use std::{fmt::Display, mem};

use crate::{move_generator::MoveList, position::Position, bitboard::Bitboard};

use PieceType::*;
use Color::*;

////////////////////////////////////////
/// General                          ///
////////////////////////////////////////

#[derive(Copy, Clone, PartialEq)]
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
}

impl Square {
    const SQUARE_STRINGS: [&str; 64] = [
        "a8",  "b8",  "c8",  "d8",  "e8",  "f8",  "g8",  "h8",
        "a7",  "b7",  "c7",  "d7",  "e7",  "f7",  "g7",  "h7",
        "a6",  "b6",  "c6",  "d6",  "e6",  "f6",  "g6",  "h6",
        "a5",  "b5",  "c5",  "d5",  "e5",  "f5",  "g5",  "h5",
        "a4",  "b4",  "c4",  "d4",  "e4",  "f4",  "g4",  "h4",
        "a3",  "b3",  "c3",  "d3",  "e3",  "f3",  "g3",  "h3",
        "a2",  "b2",  "c2",  "d2",  "e2",  "f2",  "g2",  "h2",
        "a1",  "b1",  "c1",  "d1",  "e1",  "f1",  "g1",  "h1",
    ];
}

impl From<u8> for Square {
    #[inline(always)]
    fn from(square: u8) -> Self {
        unsafe { mem::transmute::<u8, Square>(square) }
    }
}

impl From<&str> for Square {
    fn from(str: &str) -> Self {
        let chars = str.as_bytes();
        let x = chars[0] - 97 as u8;
        let y = 8 - (chars[1] as char).to_digit(10).unwrap() as u8;
        Square::from(8 * y + x)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::SQUARE_STRINGS[*self as usize])
    }
}

#[derive(Clone, Copy)]
pub enum CastlingAbility {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8
}

impl CastlingAbility {
    /// Gets the castling mask. This consists of the squares between the king and the rook, including both
    pub fn mask(&self) -> Bitboard {
        Bitboard::from(match self {
            CastlingAbility::WhiteKingSide =>  CASTLING_MASKS[0],
            CastlingAbility::WhiteQueenSide => CASTLING_MASKS[1],
            CastlingAbility::BlackKingSide =>  CASTLING_MASKS[2],
            CastlingAbility::BlackQueenSide => CASTLING_MASKS[3],
        })
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::White => "White",
            Color::Black => "Black",
        })
    }
}

impl Color {
    pub fn is_white(&self) -> bool {
        *self == White
    }

    pub fn is_black(&self) -> bool {
        *self == Black
    }

    pub fn piece_offset(&self) -> usize {
        match self {
            White => 0,
            Black => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Default for PieceType {
    fn default() -> Self {
        Self::Pawn
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Pawn =>   "Pawn",
            Knight => "Knight",
            Bishop => "Bishop",
            Rook =>   "Rook",
            Queen =>  "Queen",
            King =>   "King",
        })
    }
}

pub fn char_to_piece(char: char) -> Result<(Color, PieceType), ()> {
    let color = if char.is_uppercase() { White } else { Black };
    let piece = match char.to_ascii_uppercase() {
        'P' => Pawn,
        'R' => Rook,
        'N' => Knight,
        'B' => Bishop,
        'Q' => Queen,
        'K' => King,
        _ => return Err(())
    };

    Ok((color, piece))
}

pub fn piece_to_char(color: Color, piece_type: PieceType) -> char {
    let mut c = match piece_type {
        Pawn =>   'P',
        Knight => 'N',
        Bishop => 'B',
        Rook =>   'R',
        Queen =>  'Q',
        King =>   'K',
    };

    if color.is_black() {
        c = c.to_ascii_lowercase()
    }

    c
}

pub const PIECE_STRINGS: [&str; 13] = ["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k", "None"];

pub fn opposite_color(color: Color) -> Color {
    if color == Color::White { Color::Black } else { Color::White }
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

pub const ISOLATED_MASKS: [u64; 64] = generate_isolated_pawn_masks();

pub const WHITE_PASSED_PAWN_MASKS: [u64; 64] = generate_white_passed_pawn_masks();
pub const BLACK_PASSED_PAWN_MASKS: [u64; 64] = generate_black_passed_pawn_masks();

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

#[test]
pub fn test() {
    let a = Bitboard::from(CASTLING_MASKS[0]);
    let b = Bitboard::from(CASTLING_MASKS[1]);
    let c = Bitboard::from(CASTLING_MASKS[2]);
    let d = Bitboard::from(CASTLING_MASKS[3]);

    let pos = Position::new_from_start_pos();

    //println!("{}", pos.get_color_bitboard(White));

    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
}