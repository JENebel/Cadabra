use std::{fmt::Display, mem};

use super::*;
use MoveType::*;

#[repr(u8)]
pub enum MoveType {
    Quiet = 0,
    Capture = 1 << 0,
    DoublePush = 1 << 1,
    Enpassant = 1 << 2,
    CastleKingSide = 1 << 3,
    CastleQueenSide = 1 << 4,
    Promotion = 1 << 5,
    CapturePromotion = Capture as u8 | Promotion as u8
}

impl From<u16> for MoveType {
    fn from(value: u16) -> Self {
        unsafe { mem::transmute(value as u8) }
    }
}

/// 32 bit move representation:\
/// 00000000000  -  000000 - 000000 - 000        - 0      - 0     - 0      - 0            - 0            - 0     \ 
/// score        -  src    - dst    - piece/prom - is_cap - is_dp - is_enp - is_castle_ks - is_castle_qs - is_prom
///
/// Note that the same bits are used for promotion and the moved piece.\
/// This is ok as we know it is a pawn if we are promoting.
#[derive(Copy, Clone)]
pub struct Move {
    data: u32
}

impl Move {
    pub fn new(src: u8, dst: u8, piece: PieceType, move_type: MoveType) -> Self {
        Self {
            data: move_type as u32
                | (piece as u32) << 6
                | (src as u32) << 12
                | (dst as u32) << 18
        }
    }
    
    pub fn new_normal(src: u8, dst: u8, piece: PieceType, is_capture: bool) -> Self {
        Self::new(src, dst, piece, unsafe { mem::transmute(is_capture) })
    }

    pub fn new_promotion(src: u8, dst: u8, promotion: PieceType, is_capture: bool) -> Self {
        Self::new(src, dst, promotion, unsafe { mem::transmute(mem::transmute::<bool, u8>(is_capture) | Promotion as u8) })
    }

    pub fn piece(&self) -> PieceType {
        PieceType::from(((self.data >> 6) & 0b111111) as u8)
    }

    pub fn src(&self) -> u8 {
        ((self.data >> 12) & 0b111111) as u8
    }

    pub fn dst(&self) -> u8 {
        ((self.data >> 18) & 0b111111) as u8
    }

    /// Note that EnpassantCapture(sq) is handled seperately, and is not considered a 'capture' here.
    pub fn is_capture(&self) -> bool {
        (self.data & Capture as u32) != 0
    }

    pub fn is_double_push(&self) -> bool {
        (self.data & DoublePush as u32) != 0
    }

    pub fn is_enpassant(&self) -> bool {
        (self.data & Enpassant as u32) != 0
    }

    pub fn is_castle_ks(&self) -> bool {
        (self.data & CastleKingSide as u32) != 0
    }

    pub fn is_castle_qs(&self) -> bool {
        (self.data & CastleQueenSide as u32) != 0
    }

    pub fn is_promotion(&self) -> bool {
        (self.data & Promotion as u32) != 0
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", Square::from(self.src()), Square::from(self.dst()))?;

        if self.is_promotion() {
            write!(f, "{}", piece_char(Color::Black, self.piece()))?;
        };

        Ok(())
    }
}