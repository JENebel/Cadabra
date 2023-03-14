use std::fmt::Display;

use super::*;

use Color::*;
use PieceType::*;

#[derive(Clone, Copy)]
pub struct Position {
    pub bitboards: [Bitboard; 12],

    //3 occupancy bitboards
    pub color_occupancies: [Bitboard; 2],
    pub all_occupancies:   Bitboard,

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
            all_occupancies:   Bitboard::EMPTY,
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

    pub fn fen_string(&self) -> String {
        fn piece_at(pos: &Position, square: u8) -> Option<(Color, PieceType)> {
            if pos.bb(White, Pawn).get_bit(square) {
                Some((White, Pawn))
            } else if pos.bb(Black, Pawn).get_bit(square) {
                Some((Black, Pawn))
            } else if pos.bb(White, Knight).get_bit(square) {
                Some((White, Knight))
            } else if pos.bb(Black, Knight).get_bit(square) {
                Some((Black, Knight))
            } else if pos.bb(White, Bishop).get_bit(square) {
                Some((White, Bishop))
            } else if pos.bb(Black, Bishop).get_bit(square) {
                Some((Black, Bishop))
            } else if pos.bb(White, Rook).get_bit(square) {
                Some((White, Rook))
            } else if pos.bb(Black, Rook).get_bit(square) {
                Some((Black, Rook))
            } else if pos.bb(White, Queen).get_bit(square) {
                Some((White, Queen))
            } else if pos.bb(Black, Queen).get_bit(square) {
                Some((Black, Queen))
            } else if pos.bb(White, King).get_bit(square) {
                Some((White, King))
            } else if pos.bb(Black, King).get_bit(square) {
                Some((Black, King))
            } else {
                None
            }
        }

        let mut pieces = String::new();
        for r in 0..8 {
            let mut since = 0;

            for f in 0..8 {
                if let Some(p) = piece_at(self, r * 8 + f) {
                    if since > 0 {
                        pieces = format!("{pieces}{since}");
                        since = 0;
                    }
                    pieces = format!("{pieces}{}", piece_char(p.0, p.1));
                } else {
                    since += 1
                }
            }

            if since > 0 {
                pieces = format!("{pieces}{since}");
            }

            if r != 7 {
                pieces = format!("{pieces}/");
            }
        }

        let color = match self.active_color {
            White => 'w',
            Black => 'b',
        };

        let castling = self.castling_ability;

        let enpassant = if self.enpassant_square.is_not_empty() {
            format!("{}", Square::from(self.enpassant_square.least_significant()))
        } else {
            "-".to_string()
        };

        let half_moves = self.half_moves;
        let full_moves = self.full_moves;

        format!("{pieces} {color} {castling} {enpassant} {half_moves} {full_moves}")
    }

    #[inline(always)]
    pub fn bb(&self, color: Color, piece_type: PieceType) -> Bitboard {
        self.bitboards[piece_type.index(color)]
    }

    #[inline(always)]
    pub fn color_bb(&self, color: Color) -> Bitboard {
        self.color_occupancies[color as usize]
    }

    #[inline(always)]
    pub fn place_piece(&mut self, color: Color, piece_type: PieceType, square: u8) {
        self.bitboards[piece_type.index(color)].set_bit(square);
        self.color_occupancies[color as usize].set_bit(square);
        self.all_occupancies.set_bit(square);
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, color: Color, piece_type: PieceType, square: u8) {
        self.bitboards[piece_type.index(color)].unset_bit(square);
        self.color_occupancies[color as usize].unset_bit(square);
        self.all_occupancies.unset_bit(square);
    }

    /// Gets the position of the king of the given color
    #[inline(always)]
    pub fn king_position(&self, color: Color) -> u8 {
        self.bb(color, King).least_significant()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n  ┌────┬────┬────┬────┬────┬────┬────┬────┐")?;
        for y in 0..8 {
            write!(f, "{} │", format!("{}", 8-y ).as_str())?;
            for x in 0..8 {
                if let Some(piece_index) = (0..=11).find(|i| self.bitboards[*i].get_bit(8*y+x)) {
                    write!(f, " {}{} ", PIECE_STRINGS[piece_index], if piece_index < 6 {"."} else {" "})?;
                } else {
                    write!(f, "    ")?;
                }
                
                if x != 7 { write!(f, "│")? };
            }
            writeln!(f, "│")?;
            if y != 7 { writeln!(f, "  ├────┼────┼────┼────┼────┼────┼────┼────┤")? };
        }
        writeln!(f, "  └────┴────┴────┴────┴────┴────┴────┴────┘")?;
        writeln!(f, "    a    b    c    d    e    f    g    h\n")?;

        writeln!(f, "{}", self.fen_string())
    }
}