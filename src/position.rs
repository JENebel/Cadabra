use std::fmt::Display;

use crate::{bitboard::*, definitions::*};

use Color::*;
use PieceType::*;
use CastlingAbility::*;

#[derive(Clone, Copy)]
pub struct Position {
    pub bitboards: [Bitboard; 12],

    //3 occupancy bitboards
    pub color_occupancies: [Bitboard; 2],
    pub all_occupancies:   Bitboard,

    pub active_color: Color,
    pub enpassant_square: Bitboard,
    pub castling_ability: u8,

    pub full_moves: u16,
    pub half_moves: u8,
    pub zobrist_hash: u64,
}

impl Position {
    pub fn start_pos() -> Self {
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(input: &str) -> Result<Self, &str> {
        let fen = input.trim();
        let mut split = fen.split(' ').peekable();

        let mut bitboards: [Bitboard; 12] =  [Default::default(); 12];
        let mut color_occupancies: [Bitboard; 2] = [Default::default(); 2];
        let mut all_occupancies: Bitboard = Default::default();

        let mut i = 0;

        if split.peek().is_none() { return Err("String was emmpty") }
        let board_str = split.next().unwrap();

        for char in board_str.chars() {
            if char.is_numeric(){
                for _i in 0..char.to_digit(10).unwrap_or(0) {
                    i += 1;
                }
            }
            else if char != '/' {
                if let Ok((color, piece_type)) = char_to_piece(char) {
                    bitboards[piece_type.index(color)].set_bit(i);
                    color_occupancies[color as usize].set_bit(i)
                } else {
                    return Err("Illegal character")
                }
                
                all_occupancies.set_bit(i);

                i+=1;
            }
        }

        if split.peek().is_none() { return Err("Unexteced end") }
        let active_str = split.next().unwrap();
        let active_color = if active_str == "w" { Color::White } else { Color::Black };

        let castling_str =  if split.peek().is_some() { split.next().unwrap() } else { "" };
        let mut castling_ability: u8 = 0;
        if castling_str.contains('K') {castling_ability = castling_ability | WhiteKingSide as u8 }
        if castling_str.contains('Q') {castling_ability = castling_ability | WhiteQueenSide as u8}
        if castling_str.contains('k') {castling_ability = castling_ability | BlackKingSide as u8}
        if castling_str.contains('q') {castling_ability = castling_ability | BlackQueenSide as u8}

        let enpassant_str = if split.peek().is_some() { split.next().unwrap() } else { "-" };
        let enpassant_square: Bitboard = if enpassant_str != "-" { Bitboard(1 << Square::from(enpassant_str) as u8) } else { Bitboard::EMPTY };

        let half_moves: u8 =  if split.peek().is_some() { split.next().unwrap().parse::<u8>().unwrap()  } else { 0 };
        let full_moves: u16 = if split.peek().is_some() { split.next().unwrap().parse::<u16>().unwrap() } else { 0 };

        let mut pos = Self { 
            bitboards,
            color_occupancies,
            all_occupancies,

            active_color,
            castling_ability,
            enpassant_square,

            full_moves,
            half_moves,
            zobrist_hash: u64::default(),
        };
        
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
                let square = r * 8 + f;
                if let Some(p) = piece_at(self, square) {
                    if since > 0 {
                        pieces = format!("{pieces}{since}");
                        since = 0;
                    }
                    pieces = format!("{pieces}{}", piece_to_char(p.0, p.1));
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

        let castling = self.castling_ability_string();

        let enpassant = if self.enpassant_square.is_not_empty() {
            format!("{}", Square::from(self.enpassant_square.least_significant()))
        } else {
            "-".to_string()
        };

        let half_moves = self.half_moves;
        let full_moves = self.full_moves;

        format!("{pieces} {color} {castling} {enpassant} {half_moves} {full_moves}")
    }

    fn castling_ability_string(&self) -> String {
        if self.castling_ability == 0 {
            return '-'.to_string()
        }
        
        let mut result = String::new();
        if self.castling_ability & WhiteKingSide   as u8 != 0  { result += "K" }
        if self.castling_ability & WhiteQueenSide  as u8 != 0  { result += "Q" }
        if self.castling_ability & BlackKingSide   as u8 != 0  { result += "k" }
        if self.castling_ability & BlackQueenSide  as u8 != 0  { result += "q" }
        result
    }

    pub fn update_castling_rights(&mut self, src: u8, dst: u8) {
        self.castling_ability &= CASTLING_RIGHTS[src as usize] & CASTLING_RIGHTS[dst as usize];
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