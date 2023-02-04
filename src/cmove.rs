use std::{fmt::Display};

use crate::definitions::*;

#[derive(Copy, Clone, PartialEq)]
pub enum MoveType {
    Quiet,
    Capture,
    Promotion(PieceType),
    CapturePromotion(PieceType),
    CastleQueenSide,
    CastleKingSide,
    /// The square skipped
    DoublePush(Square),
    
    /// The square of the captured piece
    EnpassantCapture(Square),
}

impl Default for MoveType {
    fn default() -> Self {
        MoveType::Quiet
    }
}

impl Display for MoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MoveType::Quiet => String::new(),
            MoveType::Capture => format!("Capture"),
            MoveType::Promotion(promotion) => format!("Promote to {promotion}"),
            MoveType::CapturePromotion(promotion) => format!("Capture and promote to {promotion}"),
            MoveType::CastleQueenSide => format!("Castle QS"),
            MoveType::CastleKingSide => format!("Castle KS"),
            MoveType::DoublePush(_) => format!("Double pawn push"),
            MoveType::EnpassantCapture(_) => format!("Enpassant capture"),
        })
    }
}

#[derive(Copy, Clone, Default)]
pub struct Move {
    pub from_sq: u8,
    pub to_sq: u8,
    pub piece: PieceType,
    pub move_type: MoveType,

    pub score: u16,
}

impl Move {
    pub fn new_normal(from_sq: u8, to_sq: u8, piece: PieceType, is_capture: bool) -> Self {
        let move_type = match is_capture {
            true => MoveType::Capture,
            false => MoveType::Quiet,
        };

        Self { from_sq, to_sq, piece, move_type, score: 0 }
    }

    pub fn new_promotion(from_sq: u8, to_sq: u8, promotion: PieceType, is_capture: bool) -> Self {
        let move_type = match is_capture {
            true => MoveType::CapturePromotion(promotion),
            false => MoveType::Promotion(promotion),
        };

        Self { from_sq, to_sq, piece: PieceType::Pawn, move_type, score: 0 }
    }

    pub fn new_custom(from_sq: u8, to_sq: u8, piece: PieceType, move_type: MoveType) -> Self {
            Self {
                from_sq,
                to_sq,
                piece,
                move_type,
                score: u16::MAX,
            }
    }

    /// Note that EnpassantCapture(sq) is handled seperately, and is not considered a 'capture' here.
    pub fn is_capture(&self) -> bool {
        match self.move_type {
            MoveType::Capture | MoveType::CapturePromotion(_) => true,
            _ => false
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("{}: {} -> {}", self.piece, Square::from(self.from_sq), Square::from(self.to_sq));

        if self.move_type != MoveType::Quiet {
            res = format!("{res} - {}", self.move_type)
        }

        write!(f, "{res}")
    }
}