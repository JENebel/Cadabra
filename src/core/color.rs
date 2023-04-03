use std::fmt::Display;

use Color::*;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
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
    pub fn from_str(str: &str) -> Result<Color, String> {
        match str {
            "w" => Ok(White),
            "b" => Ok(Black),
            _ => Err(format!("Illegal color string: '{str}'. Expected 'w' or 'b'"))
        }
    }

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

    pub fn opposite(&self) -> Color {
        if self.is_white() { Color::Black } else { Color::White }
    }
}