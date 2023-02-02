use crate::{zobrist_constants::*, bitboard::*, attack_tables::*, definitions::*};

use Color::*;
use Piece::*;
use PieceType::*;

#[derive(Clone, Copy)]
pub struct Position {
    bitboards: [Bitboard; 12],

    //3 occupancy bitboards
    pub white_occupancies: Bitboard,
    pub black_occupancies: Bitboard,
    pub all_occupancies:   Bitboard,

    pub active_color: Color,
    pub enpassant_square: Option<Square>,
    pub castling_ability: u8,

    pub full_moves: u16,
    pub half_moves: u8,
    pub zobrist_hash: u64,

    // Repetition table should be included for make/unmake
}

impl Position {
    pub fn pretty_print(&self) {
        println!("\n  ┌────┬────┬────┬────┬────┬────┬────┬────┐");
        for y in 0..8 {
            print!("{} │", format!("{}", 8-y ).as_str());
            for x in 0..8 {
                let piece_index = (0..11).find(|i| self.bitboards[*i].get_bit(8*y+x)).unwrap();
                print!(" {}{} ", PIECE_STRINGS[piece_index], if piece_index < 6 {" "} else {"."});
                if x != 7 { print!("│") };
            }
            println!("│");
            if y != 7 { println!("  ├────┼────┼────┼────┼────┼────┼────┼────┤")};
        }
        println!("  └────┴────┴────┴────┴────┴────┴────┴────┘");
        println!("    a    b    c    d    e    f    g    h\n");

        print!("   Active:     {}",     self.active_color);
        println!("\tFull moves: {}",      self.full_moves);
        if let Some(enpassant) = self.enpassant_square {
            print!("   Enpassant:  {}",     SQUARE_STRINGS[enpassant as usize]);
        }
        println!("\tHalf moves: {}",      self.half_moves);
        print!("   Castling:   {}  ",   self.castling_ability_string());
        println!("\tZobrist:   {:#0x}\n", self.zobrist_hash);
    }

    fn castling_ability_string(&self) -> String {
        let mut result = String::new();
        if self.castling_ability & CastlingAbility::WhiteKingSide   as u8 != 0  { result += "K" }
        if self.castling_ability & CastlingAbility::WhiteQueenSide  as u8 != 0  { result += "Q" }
        if self.castling_ability & CastlingAbility::BlackKingSide   as u8 != 0  { result += "k" }
        if self.castling_ability & CastlingAbility::BlackQueenSide  as u8 != 0  { result += "q" }
        result
    }

    pub fn new_from_start_pos() -> Self {
        Position::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn new_from_fen(input: &str) -> Option<Self> {
        let fen = input.trim();
        let mut split = fen.split(' ').peekable();

        let mut bitboards: [Bitboard; 12] =  [Default::default(); 12];
        let mut white_occupancies: Bitboard = Default::default();
        let mut black_occupancies: Bitboard = Default::default();
        let mut all_occupancies:   Bitboard = Default::default();

        let mut i = 0;

        if split.peek().is_none() { return None }
        let board_str = split.next().unwrap();

        for char in board_str.chars() {
            if char.is_numeric(){
                for _i in 0..char.to_digit(10).unwrap_or(0) {
                    i += 1;
                }
            }
            else if char != '/' {
                let piece = char_to_piece(char);
                if piece.is_none() { return None };
                bitboards[char_to_piece(char).unwrap() as usize].set_bit(i);
                all_occupancies.set_bit(i);
                if char.is_uppercase() { white_occupancies.set_bit(i) } else { black_occupancies.set_bit(i) };

                i+=1;
            }
        }

        if split.peek().is_none() { return None }
        let active_str = split.next().unwrap();
        let active_color = if active_str == "w" { Color::White } else { Color::Black };

        let castling_str =  if split.peek().is_some() { split.next().unwrap() } else { "" };
        let mut castling_ability: u8 = 0;
        if castling_str.contains('K') {castling_ability = castling_ability | CastlingAbility::WhiteKingSide as u8 }
        if castling_str.contains('Q') {castling_ability = castling_ability | CastlingAbility::WhiteQueenSide as u8}
        if castling_str.contains('k') {castling_ability = castling_ability | CastlingAbility::BlackKingSide as u8}
        if castling_str.contains('q') {castling_ability = castling_ability | CastlingAbility::BlackQueenSide as u8}

        let enpassant_str = if split.peek().is_some() { split.next().unwrap() } else { "-" };
        let enpassant_square: Option<Square> = if enpassant_str != "-" { Some(square_from_string(enpassant_str)) } else { None };

        let half_moves: u8 =  if split.peek().is_some() { split.next().unwrap().parse::<u8>().unwrap()  } else { 0 };
        let full_moves: u16 = if split.peek().is_some() { split.next().unwrap().parse::<u16>().unwrap() } else { 0 };

        let mut pos = Self { 
            bitboards,
            white_occupancies,
            black_occupancies,
            all_occupancies,

            active_color,
            castling_ability,
            enpassant_square,

            full_moves,
            half_moves,
            zobrist_hash: u64::default(),
        };
        
        pos.generate_zobrist_hash();

        Some(pos)
    }

    #[inline(always)]
    pub fn get_piece_color_bitboard(&self, piece_type: PieceType, color: Color) -> Bitboard {
        let index = if color == Color::White {
            piece_type as usize
        } else {
            piece_type as usize + 6
        };
        self.bitboards[index]
    }

    #[inline(always)]
    pub fn get_piece_bitboard(&self, piece: Piece) -> Bitboard {
        self.bitboards[piece as usize]
    }

    #[inline(always)]
    pub fn get_bitboard(&self, piece_index: usize) -> Bitboard {
        self.bitboards[piece_index]
    }

    #[inline(always)]
    pub fn get_color_bitboard(&self, color: Color) -> Bitboard {
        if color == Color::White {
            self.white_occupancies
        } else {
            self.black_occupancies
        }
    }

    /// Creates a zobrist hash from scratch for the current position
    fn generate_zobrist_hash(&mut self) {
        let mut hash = 0;

        for piece in 0..12 {
            let mut bb = self.bitboards[piece];
            while let Some(square) = bb.extract_bit() {
                hash ^= PIECE_KEYS[piece][square as usize];
            }
        }

        hash ^= CASTLE_KEYS[self.castling_ability as usize];
        
        if self.active_color == Black {
            hash ^= SIDE_KEY;
        }

        if let Some(enpassant) = self.enpassant_square {
            hash ^= ENPASSANT_KEYS[enpassant as usize];
        }

        self.zobrist_hash = hash
    }

    #[inline(always)]
    /// Indicates whether a square is attacked
    pub fn is_square_attacked(&self, square: u8, by_color: Color) -> bool {
        get_pawn_attack_table   (square, opposite_color(by_color)) .and(self.get_piece_color_bitboard(Pawn,   by_color)).is_not_empty() ||
        get_knight_attack_table (square)                           .and(self.get_piece_color_bitboard(Knight, by_color)).is_not_empty() ||
        get_king_attack_table   (square)                           .and(self.get_piece_color_bitboard(King,   by_color)).is_not_empty() ||
        get_rook_attack_table   (square, self.all_occupancies)     .and(self.get_piece_color_bitboard(Rook,   by_color)).is_not_empty() ||
        get_bishop_attack_table (square, self.all_occupancies)     .and(self.get_piece_color_bitboard(Bishop, by_color)).is_not_empty() ||
        get_queen_attack_table  (square, self.all_occupancies)     .and(self.get_piece_color_bitboard(Queen,  by_color)).is_not_empty()
    }

    /// Gets the position of the king of the given color
    #[inline(always)]
    pub fn king_position(&self, color: Color) -> u8 {
        self.get_piece_color_bitboard(King, color).least_significant()
    }

    #[inline(always)]
    pub fn is_in_check(&self, color: Color) -> bool {
        self.is_square_attacked(self.king_position(color), opposite_color(color))
    }
}