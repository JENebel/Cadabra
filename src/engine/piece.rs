use std::{fmt::Display, mem};

use crate::engine::Color::{*, self};
use PieceType::*;

pub const PIECE_STRINGS: [&str; 13] = ["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k", "None"];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
    Empty = u8::MAX
}

impl From<u8> for PieceType {
    fn from(value: u8) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl Default for PieceType {
    fn default() -> Self {
        Pawn
    }
}

impl PieceType {
    /// Calculates the bitboard index of the piece, given its offset
    pub fn index(&self, color: Color) -> usize {
        debug_assert!(self != &Empty);
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
            Empty =>  "Empty"
        })
    }
}

pub fn index_to_piece(index: usize) -> (Color, PieceType) {
    debug_assert!(index < 12);
    let color = unsafe { mem::transmute::<u8, Color>((index / 6) as u8) };
    let piece = unsafe { mem::transmute::<u8, PieceType>((index % 6) as u8) };
    (color, piece)
}

pub fn char_to_piece(char: char) -> Result<(Color, PieceType), String> {
    let color = if char.is_uppercase() { White } else { Black };
    let piece = match char.to_ascii_uppercase() {
        'P' => Pawn,
        'R' => Rook,
        'N' => Knight,
        'B' => Bishop,
        'Q' => Queen,
        'K' => King,
        _ => return Err(format!("Illegal piece char: '{char}'"))
    };

    Ok((color, piece))
}

pub fn piece_char(color: Color, piece_type: PieceType) -> char {
    let mut c = match piece_type {
        Pawn =>   'P',
        Knight => 'N',
        Bishop => 'B',
        Rook =>   'R',
        Queen =>  'Q',
        King =>   'K',
        Empty =>  ' '
    };

    if color.is_black() {
        c = c.to_ascii_lowercase()
    }

    c
}