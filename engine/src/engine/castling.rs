const ATTACKED_CASTLING_MASKS: [u64; 4] = generate_attacked_castling_masks();
const OPEN_CASTLING_MASKS: [u64; 4] = generate_open_castling_masks();

const CASTLING_RIGHTS: [u8; 64] = [
    7, 15, 15, 15,  3, 15, 15, 11,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   13, 15, 15, 15, 12, 15, 15, 14
];

use std::fmt::Display;

use CastlingSide::*;

#[derive(Copy, Clone)]
pub struct CastlingAbility {
    pub byte: u8
}

impl CastlingAbility {
    pub fn from_str(castling_str: &str) -> Result<CastlingAbility, String> {
        let mut byte: u8 = 0;

        if castling_str == "-" {
            return Ok(Self{byte})
        }

        for c in castling_str.chars() {
            let bit = match c {
                'K' => WhiteKingSide as u8,
                'Q' => WhiteQueenSide as u8,
                'k' => BlackKingSide as u8,
                'q' => BlackQueenSide as u8,
                _ => return Err(format!("Illegal char in castling ability string: '{c}'"))
            };

            // If bit already set
            if byte & bit != 0 {
                return Err(format!("Duplicate char in castling string: {bit}"))
            }

            byte |= bit;
        }

        Ok(Self{byte})
    }

    pub fn is_side_available(&self, side: CastlingSide) -> bool {
        self.byte & (side as u8) != 0
    }

    pub fn update(&mut self, src: u8, dst: u8) {
        self.byte &= CASTLING_RIGHTS[src as usize] & CASTLING_RIGHTS[dst as usize];
    }
}

impl Display for CastlingAbility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.byte == 0 {
            write!(f, "-")?;
            return Ok(());
        }
        
        if self.byte & WhiteKingSide   as u8 != 0  { write!(f, "K")? }
        if self.byte & WhiteQueenSide  as u8 != 0  { write!(f, "Q")? }
        if self.byte & BlackKingSide   as u8 != 0  { write!(f, "k")? }
        if self.byte & BlackQueenSide  as u8 != 0  { write!(f, "q")? }
        
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum CastlingSide {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8
}

impl CastlingSide {
    /// Gets the castling mask. This consists of the squares between the king and the rook, including both
    pub fn attacked_mask(&self) -> u64 {
        match self {
            CastlingSide::WhiteKingSide =>  ATTACKED_CASTLING_MASKS[0],
            CastlingSide::WhiteQueenSide => ATTACKED_CASTLING_MASKS[1],
            CastlingSide::BlackKingSide =>  ATTACKED_CASTLING_MASKS[2],
            CastlingSide::BlackQueenSide => ATTACKED_CASTLING_MASKS[3],
        }
    }

    /// Gets the castling mask. This consists of the squares between the king and the rook, including both
    pub fn open_mask(&self) -> u64 {
        match self {
            CastlingSide::WhiteKingSide =>  OPEN_CASTLING_MASKS[0],
            CastlingSide::WhiteQueenSide => OPEN_CASTLING_MASKS[1],
            CastlingSide::BlackKingSide =>  OPEN_CASTLING_MASKS[2],
            CastlingSide::BlackQueenSide => OPEN_CASTLING_MASKS[3],
        }
    }
}

const fn generate_open_castling_masks() -> [u64; 4] {
    let mut masks = [0; 4];

    // White
    masks[0] |= 1 << 62 | 1 << 61;
    masks[1] |= 1 << 59 | 1 << 58 | 1 << 57;

    // Black
    masks[2] |= 1 << 5 | 1 << 6;
    masks[3] |= 1 << 1 | 1 << 2 | 1 << 3;
    masks
}

const fn generate_attacked_castling_masks() -> [u64; 4] {
     let mut masks = [0; 4];

    // White
    masks[0] |= 1 << 62 | 1 << 61;
    masks[1] |= 1 << 59 | 1 << 58;

    // Black
    masks[2] |= 1 << 5 | 1 << 6;
    masks[3] |= 1 << 2 | 1 << 3;
    masks
}