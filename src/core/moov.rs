use std::{fmt::Display};

use super::*;
use MoveType::*;

// Do not change this ddefinition. Constants depend on it!
#[derive(Copy, Clone, PartialEq)]
pub enum MoveType {
    Quiet,
    Capture,
    Promotion(PieceType),
    CapturePromotion(PieceType),
    CastleKingSide,
    CastleQueenSide,
    DoublePush,
    
    EnpassantCapture,
}

impl MoveType {
    /// Note that EnpassantCapture(sq) is handled seperately, and is not considered a 'capture' here.
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        match self {
            Capture | CapturePromotion(_) => true,
            _ => false
        }
    }

    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        match self {
            Promotion(_) | CapturePromotion(_) => true,
            _ => false
        }
    }

    #[inline(always)]
    pub fn is_castling(&self) -> bool {
        match self {
            CastleKingSide | CastleQueenSide => true,
            _ => false
        }
    }
}

impl Default for MoveType {
    #[inline(always)]
    fn default() -> Self {
        Quiet
    }
}

impl Display for MoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Quiet => String::new(),
            Capture => format!("Capture"),
            Promotion(promotion) => format!("Promote to {promotion}"),
            CapturePromotion(promotion) => format!("Capture and promote to {promotion}"),
            CastleQueenSide => format!("Castle QS"),
            CastleKingSide => format!("Castle KS"),
            DoublePush => format!("Double pawn push"),
            EnpassantCapture => format!("Enpassant capture"),
        })
    }
}

#[derive(Copy, Clone, Default)]
pub struct Move {
    pub src: u8,
    pub dst: u8,
    pub piece: PieceType,
    pub move_type: MoveType,

    pub score: u16,
}

impl Move {
    #[inline(always)]
    pub fn new(src: u8, dst: u8, piece: PieceType, move_type: MoveType) -> Self {
        Self {
            src,
            dst,
            piece,
            move_type,
            score: 0,
        }
    }

    #[inline(always)]
    pub fn new_normal(src: u8, dst: u8, piece: PieceType, is_capture: bool) -> Self {
        let move_type = match is_capture {
            true => Capture,
            false => Quiet,
        };

        Self { src: src, dst: dst, piece, move_type, score: 0 }
    }

    #[inline(always)]
    pub fn new_promotion(src: u8, dst: u8, promotion: PieceType, is_capture: bool) -> Self {
        let move_type = match is_capture {
            true => CapturePromotion(promotion),
            false => Promotion(promotion),
        };

        Self { src, dst, piece: PieceType::Pawn, move_type, score: 0 }
    }

    /// Note that EnpassantCapture(sq) is handled seperately, and is not considered a 'capture' here.
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        self.move_type.is_capture()
    }

    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        self.move_type.is_promotion()
    }

    #[inline(always)]
    pub fn is_castling(&self) -> bool {
        self.move_type.is_castling()
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("{}{}", Square::from(self.src), Square::from(self.dst));

        if let Promotion(p) | CapturePromotion(p) = self.move_type {
            res = format!("{res}{}", piece_char(Color::Black, p));
        }

        write!(f, "{res}")
    }
}