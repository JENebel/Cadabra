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

impl From<u16> for MoveType {
    #[inline(always)]
    fn from(move_type_const: u16) -> Self {
        match move_type_const {
            0 =>  Quiet,
            1 =>  Capture,
            2 =>  Promotion(PieceType::Queen),
            3 =>  Promotion(PieceType::Rook),
            4 =>  Promotion(PieceType::Bishop),
            5 =>  Promotion(PieceType::Knight),
            6 =>  CapturePromotion(PieceType::Queen),
            7 =>  CapturePromotion(PieceType::Rook),
            8 =>  CapturePromotion(PieceType::Bishop),
            9 =>  CapturePromotion(PieceType::Knight),
            10 => CastleKingSide,
            11 => CastleQueenSide,
            12 => DoublePush,
            13 => EnpassantCapture,
            _ => panic!("Illegal move_type!")
        }
    }
}

impl MoveType {
    /// Note that EnpassantCapture(sq) is handled seperately, and is not considered a 'capture' here.
    pub fn is_capture(&self) -> bool {
        match self {
            MoveType::Capture | MoveType::CapturePromotion(_) => true,
            _ => false
        }
    }

    pub fn is_promotion(&self) -> bool {
        match self {
            MoveType::Promotion(_) | MoveType::CapturePromotion(_) => true,
            _ => false
        }
    }

    pub fn is_castling(&self) -> bool {
        match self {
            MoveType::CastleKingSide | MoveType::CastleQueenSide => true,
            _ => false
        }
    }
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
            MoveType::DoublePush => format!("Double pawn push"),
            MoveType::EnpassantCapture => format!("Enpassant capture"),
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
        self.move_type.is_capture()
    }

    pub fn is_promotion(&self) -> bool {
        self.move_type.is_promotion()
    }

    pub fn is_castling(&self) -> bool {
        self.move_type.is_castling()
    }

    pub fn to_uci_string(&self) -> String {
        let mut res = format!("{}{}", Square::from(self.from_sq), Square::from(self.to_sq));

        if let Promotion(p) | CapturePromotion(p) = self.move_type {
            res = format!("{res}{}", piece_to_char(Color::Black, p));
        }

        res
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