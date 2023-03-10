use std::{fmt::Display, mem};
use super::*;

use PieceType::*;
use Color::*;

pub const PKG_NAME: &str = "Cadabra";
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub const LOOKUP_RANK: [usize; 64] =
[
    7, 7, 7, 7, 7, 7, 7, 7,
    6, 6, 6, 6, 6, 6, 6, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    4, 4, 4, 4, 4, 4, 4, 4,
    3, 3, 3, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
	0, 0, 0, 0, 0, 0, 0, 0,
];

pub const LOOKUP_FILE: [usize; 64] =
[
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
	0, 1, 2, 3, 4, 5, 6, 7,
];

pub const LOOKUP_D1: [usize; 64] =
[
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 0,
    2, 2, 2, 2, 2, 2, 1, 0,
    3, 3, 3, 3, 3, 2, 1, 0,
    4, 4, 4, 4, 3, 2, 1, 0,
    5, 5, 5, 4, 3, 2, 1, 0,
    6, 6, 5, 4, 3, 2, 1, 0,
	7, 6, 5, 4, 3, 2, 1, 0,
];

pub const LOOKUP_D2: [usize; 64] =
[
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1,
    0, 1, 2, 2, 2, 2, 2, 2,
    0, 1, 2, 3, 3, 3, 3, 3,
    0, 1, 2, 3, 4, 4, 4, 4,
    0, 1, 2, 3, 4, 5, 5, 5,
    0, 1, 2, 3, 4, 5, 6, 6,
	0, 1, 2, 3, 4, 5, 6, 7,
];

pub const CASTLING_RIGHTS: [u8; 64] = [
    7, 15, 15, 15,  3, 15, 15, 11,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   13, 15, 15, 15, 12, 15, 15, 14
];

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
    #[inline(always)]
    pub fn attacked_mask(&self) -> u64 {
        match self {
            CastlingAbility::WhiteKingSide =>  ATTACKED_CASTLING_MASKS[0],
            CastlingAbility::WhiteQueenSide => ATTACKED_CASTLING_MASKS[1],
            CastlingAbility::BlackKingSide =>  ATTACKED_CASTLING_MASKS[2],
            CastlingAbility::BlackQueenSide => ATTACKED_CASTLING_MASKS[3],
        }
    }

    /// Gets the castling mask. This consists of the squares between the king and the rook, including both
    #[inline(always)]
    pub fn open_mask(&self) -> u64 {
        match self {
            CastlingAbility::WhiteKingSide =>  OPEN_CASTLING_MASKS[0],
            CastlingAbility::WhiteQueenSide => OPEN_CASTLING_MASKS[1],
            CastlingAbility::BlackKingSide =>  OPEN_CASTLING_MASKS[2],
            CastlingAbility::BlackQueenSide => OPEN_CASTLING_MASKS[3],
        }
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
    #[inline(always)]
    pub fn is_white(&self) -> bool {
        *self == White
    }

    #[inline(always)]
    pub fn is_black(&self) -> bool {
        *self == Black
    }

    #[inline(always)]
    pub fn piece_offset(&self) -> usize {
        match self {
            White => 0,
            Black => 6,
        }
    }

    #[inline(always)]
    pub fn opposite(&self) -> Color {
        if self.is_white() { Color::Black } else { Color::White }
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
        Pawn
    }
}

impl PieceType {
    /// Calculates the bitboard index of the piece, given its offset
    #[inline(always)]
    pub fn index(&self, color: Color) -> usize {
        *self as usize + color.piece_offset()
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

pub struct Settings {
    pub threads: u8,
    pub transposition_table_mb: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self { threads: 1, transposition_table_mb: 128 }
    }
}