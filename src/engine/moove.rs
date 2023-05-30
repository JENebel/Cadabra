use std::{fmt::Display, mem};

use super::*;
use MoveType::*;
use PieceType::*;

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum MoveType {
    Quiet =             0b0000,
    Capture =           0b0001,
    Promotion =         0b0010,
    DoublePush =        0b0100,
    CastleKingSide =    0b1000,
    CastleQueenSide =   0b1100,
    Enpassant =         0b0101,
}

/// 16 bit move representation:
/// 000000 - 000000 - 00    - 0       - 0\
/// src    - dst    - extra - is_prom - is_cap
///
/// Note that the same bits are used for promotion and the moved piece.\
/// This is ok as we know it is a pawn if we are promoting.
#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub data: u16
}

impl From<u16> for Move {
    fn from(data: u16) -> Self {
        Self { data }
    }
}

impl Move {
    pub const NULL: Move = Move { data: 0 };

    fn new_internal(src: u8, dst: u8, move_type: u8) -> Self {
        Self {
            data: move_type as u16
                | (src as u16) << 4
                | (dst as u16) << 10
        }
    }

    pub fn new(src: u8, dst: u8, move_type: MoveType) -> Self {
        Self::new_internal(src, dst, move_type as u8)

    }
    
    pub fn new_normal(src: u8, dst: u8, is_capture: bool) -> Self {
        Self::new_internal(src, dst, unsafe { mem::transmute(is_capture) })
    }

    pub fn new_promotion(src: u8, dst: u8, promotion: PieceType, is_capture: bool) -> Self {
        let mut move_type = match promotion {
            Knight => 0b0000,
            Bishop => 0b0100,
            Rook =>   0b1000,
            Queen =>  0b1100,
            _ => unreachable!()
        };
        move_type |= unsafe { mem::transmute::<bool, u8>(is_capture) };
        move_type |= Promotion as u8;

        Self::new_internal(src, dst, move_type)
    }

    pub fn src(&self) -> u8 {
        (self.data >> 4) as u8 & 0b111111
    }

    pub fn dst(&self) -> u8 {
        (self.data >> 10) as u8 // & 0b111111
    }

    pub fn is_capture(&self) -> bool {
        (self.data & Capture as u16) != 0
    }

    pub fn move_type(&self) -> MoveType {
        match self.data & 0b1111{
            0b0000 => Quiet,
            0b0001 => Capture,
            0b0100 => DoublePush,
            0b1000 => CastleKingSide,
            0b1100 => CastleQueenSide,
            0b0101 => Enpassant,
            _ => Promotion
        }
    }

    pub fn is_promotion(&self) -> bool {
        (self.data & Promotion as u16) != 0
    }

    pub fn is_double_push(&self) -> bool {
        (self.data as u8 & 0b1111) == DoublePush as u8
    }

    pub fn is_enpassant(&self) -> bool {
        (self.data as u8 & 0b1111) == Enpassant as u8
    }

    pub fn is_castle_ks(&self) -> bool {
        (self.data as u8 & 0b1111) == CastleKingSide as u8
    }

    pub fn is_castle_qs(&self) -> bool {
        (self.data as u8 & 0b1111) == CastleQueenSide as u8
    }

    pub fn promotion(&self) -> PieceType {
        match self.data & 0b1100 {
            0b0000 => Knight,
            0b0100 => Bishop,
            0b1000 => Rook,
            0b1100 => Queen,
            _ => unreachable!()
        }
    }

    pub fn is_null(&self) -> bool {
        self.data == Self::NULL.data
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", Square::from(self.src()), Square::from(self.dst()))?;

        if self.is_promotion() {
            write!(f, "{}", piece_char(Color::Black, self.promotion()))?;
        };

        Ok(())
    }
}