use std::{fmt::Display, mem};

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

    pub fn from_str(str: &str) -> Result<Square, String> {
        if !str.is_ascii() {
            return Err(format!("Square string contained non ASCII characters"))
        }

        let chars = str.as_bytes();

        if chars.len() != 2 {
            return Err(format!("Square string must be 2 chars but was {}", chars.len()))
        }
        
        let file = chars[0] - 97 as u8;
        if file > 7 {
            return Err(format!("Illegal file in square string"))
        }

        let rank = 8 - (chars[1] as char).to_digit(10).unwrap() as u8;
        if rank > 7 {
            return Err(format!("Illegal rank in square string"))
        }

        Ok(Square::from(8 * file + rank))
    }
}

impl From<u8> for Square {
    #[inline(always)]
    fn from(square: u8) -> Self {
        debug_assert!(square < 64);
        unsafe { mem::transmute::<u8, Square>(square) }
    }
}


impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::SQUARE_STRINGS[*self as usize])
    }
}