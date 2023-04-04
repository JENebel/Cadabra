use std::fmt::{Display, Write};

use super::*;

use Color::*;
use PieceType::*;

#[derive(Clone, Copy)]
pub struct Position {
    pub bitboards: [Bitboard; 12],

    //3 occupancy bitboards
    pub color_occupancies: [Bitboard; 2],
    pub all_occupancies:   Bitboard,

    piece_squares: [PieceType; 64],

    pub active_color: Color,
    pub enpassant_square: Bitboard,
    pub castling_ability: CastlingAbility,

    pub full_moves: u16,
    pub half_moves: u8,
    pub zobrist_hash: u64,
}

impl Position {
    pub fn start_pos() -> Self {
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(input: &str) -> Result<Self, String> {
        let mut split = input.trim().split(' ');
        
        // Get and remember board pieces string
        let board_str = match split.next() {
            Some(str) => str,
            None => return Err(format!("Expected board string in fen")),
        };

        // Active color
        let active_color = if let Some(color_str) = split.next() {
            Color::from_str(color_str)?
        } else {
            return Err(format!("Expected a color char"))
        };

        // Castling ability
        let castling_ability = CastlingAbility::from_str(split.next().unwrap_or("-"))?;

        // Enpassant square
        let mut enpassant_square = Bitboard::EMPTY;
        if let Some(enp_str) = split.next() {
            if enp_str != "-" {
                enpassant_square.set_bit(Square::from_str(enp_str)? as u8)
            }
        };

        // 50 move rule count
        let half_moves: u8 = if let Some(hm_str) = split.next() {
            match hm_str.parse() {
                Ok(i) => i,
                Err(_) => return Err(format!("Half moves was not a number")),
            }
        } else { 0 };

        // Full moves
        let full_moves: u16 = if let Some(fm_str) = split.next() {
            match fm_str.parse() {
                Ok(i) => i,
                Err(_) => return Err(format!("Full moves was not a number")),
            }
        } else { 0 };

        // Init position with empty board
        let mut pos = Position { 
            bitboards: [Bitboard::EMPTY; 12],
            color_occupancies: [Bitboard::EMPTY; 2],
            all_occupancies: Bitboard::EMPTY,
            piece_squares: [Empty; 64],
            active_color,
            enpassant_square,
            castling_ability,
            full_moves,
            half_moves,
            zobrist_hash: 0,
        };

        // Place pieces
        let mut square = 0;
        for char in board_str.chars() {
            if square > 63 {
                return Err(format!("Board from fen does not fit on the board"))
            }
            
            if let Some(i) = char.to_digit(10) {
                square += i as u8
            } else if char != '/' {
                let (color, piece_type) = char_to_piece(char)?;
                pos.place_piece(color, piece_type, square);
                square += 1;
            }
        }
        
        // Initialize zobrist
        pos.generate_zobrist_hash();

        Ok(pos)
    }

    pub fn piece_type_at(&self, square: u8) -> PieceType {
        self.piece_squares[square as usize]
    }

    pub fn piece_at(&self, square: u8) -> Option<(Color, PieceType)> {
        for p in 0..12 {
            if self.bitboards[p].get_bit(square) {
                return Some(index_to_piece(p))
            }
        }
        None
    }

    pub fn fen_string(&self) -> String {
        let mut result = String::new();
        for r in 0..8 {
            let mut since = 0;

            for f in 0..8 {
                if let Some(p) = self.piece_at(r * 8 + f) {
                    if since > 0 {
                        write!(result, "{since}").unwrap();
                        since = 0;
                    }
                    write!(result, "{}", piece_char(p.0, p.1)).unwrap();
                } else {
                    since += 1
                }
            }
            if since > 0 { write!(result, "{since}").unwrap() }
            if r < 7 { write!(result, "/").unwrap() }
        }

        write!(result, " {}", 
            match self.active_color {
                White => 'w',
                Black => 'b',
            }
        ).unwrap();

        write!(result, " {}", self.castling_ability).unwrap();

        if !self.enpassant_square.is_empty() {
            write!(result, " {}", Square::from(self.enpassant_square.least_significant())).unwrap()
        } else {
            write!(result, " -").unwrap()
        }

        write!(result, " {}", self.half_moves).unwrap();
        write!(result, " {}", self.full_moves).unwrap();

        result
    }

    pub fn bb(&self, color: Color, piece_type: PieceType) -> Bitboard {
        self.bitboards[piece_type.index(color)]
    }

    pub fn color_bb(&self, color: Color) -> Bitboard {
        self.color_occupancies[color as usize]
    }

    pub fn place_piece(&mut self, color: Color, piece_type: PieceType, square: u8) {
        self.bitboards[piece_type.index(color)].set_bit(square);
        self.color_occupancies[color as usize].set_bit(square);
        self.all_occupancies.set_bit(square);
        self.piece_squares[square as usize] = piece_type;
    }

    pub fn remove_piece(&mut self, color: Color, square: u8) {
        self.bitboards[self.piece_type_at(square).index(color)].unset_bit(square);
        self.color_occupancies[color as usize].unset_bit(square);
        self.all_occupancies.unset_bit(square);
        self.piece_squares[square as usize] = Empty;
    }

    /// Gets the position of the king of the given color
    pub fn king_position(&self, color: Color) -> u8 {
        self.bb(color, King).least_significant()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n  ┌────┬────┬────┬────┬────┬────┬────┬────┐")?;
        for y in 0..8 {
            write!(f, "{} │", 8 - y)?;
            for x in 0..8 {
                if let Some(piece_index) = (0..=11).find(|i| self.bitboards[*i].get_bit(8*y+x)) {
                    write!(f, " {}{} ", PIECE_STRINGS[piece_index], if piece_index < 6 {"."} else {" "})?;
                } else {
                    write!(f, "    ")?;
                }
                
                if x < 7 { write!(f, "│")? };
            }
            writeln!(f, "│")?;
            if y < 7 { writeln!(f, "  ├────┼────┼────┼────┼────┼────┼────┼────┤")? };
        }
        writeln!(f, "  └────┴────┴────┴────┴────┴────┴────┴────┘")?;
        writeln!(f, "    a    b    c    d    e    f    g    h\n")?;

        writeln!(f, "{}", self.fen_string())
    }
}