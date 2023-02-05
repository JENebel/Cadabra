use crate::{zobrist_constants::*, bitboard::*, attack_tables::*, definitions::*};

use Color::*;
use PieceType::*;

#[derive(Clone, Copy)]
pub struct Position {
    pub bitboards: [Bitboard; 12],

    //3 occupancy bitboards
    pub color_occupancies: [Bitboard; 2],
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
                if let Some(piece_index) = (0..=11).find(|i| self.bitboards[*i].get_bit(8*y+x)) {
                    print!(" {}{} ", PIECE_STRINGS[piece_index], if piece_index < 6 {" "} else {"."});
                } else {
                    print!("    ");
                }
                
                if x != 7 { print!("│") };
            }
            println!("│");
            if y != 7 { println!("  ├────┼────┼────┼────┼────┼────┼────┼────┤")};
        }
        println!("  └────┴────┴────┴────┴────┴────┴────┴────┘");
        println!("    a    b    c    d    e    f    g    h\n");

        print!("   Active:     {}", self.active_color);
        println!("\tFull moves: {}", self.full_moves);
        if let Some(enpassant) = self.enpassant_square {
            print!("   Enpassant:  {}", enpassant);
        }
        println!("\tHalf moves: {}", self.half_moves);
        print!("   Castling:   {}  ", self.castling_ability_string());
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

    pub fn new_from_fen(input: &str) -> Result<Self, &str> {
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
                    bitboards[Position::get_bitboard_index(color, piece_type)].set_bit(i);
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
        if castling_str.contains('K') {castling_ability = castling_ability | CastlingAbility::WhiteKingSide as u8 }
        if castling_str.contains('Q') {castling_ability = castling_ability | CastlingAbility::WhiteQueenSide as u8}
        if castling_str.contains('k') {castling_ability = castling_ability | CastlingAbility::BlackKingSide as u8}
        if castling_str.contains('q') {castling_ability = castling_ability | CastlingAbility::BlackQueenSide as u8}

        let enpassant_str = if split.peek().is_some() { split.next().unwrap() } else { "-" };
        let enpassant_square: Option<Square> = if enpassant_str != "-" { Some(Square::from(enpassant_str)) } else { None };

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

    #[inline(always)]
    pub fn get_bitboard_index(color: Color, piece_type: PieceType) -> usize {
        piece_type as usize + color.piece_offset()
    }

    #[inline(always)]
    pub fn get_bitboard(&self, color: Color, piece_type: PieceType) -> Bitboard {
        self.bitboards[Self::get_bitboard_index(color, piece_type)]
    }

    #[inline(always)]
    pub fn get_color_bitboard(&self, color: Color) -> Bitboard {
        self.color_occupancies[color as usize]
    }

    #[inline(always)]
    pub fn place_piece(&mut self, color: Color, piece_type: PieceType, square: u8) {
        self.bitboards[Self::get_bitboard_index(color, piece_type)].set_bit(square);
        self.color_occupancies[color as usize].set_bit(square);
        self.all_occupancies.set_bit(square);
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, color: Color, piece_type: PieceType, square: u8) {
        self.bitboards[Self::get_bitboard_index(color, piece_type)].unset_bit(square);
        self.color_occupancies[color as usize].unset_bit(square);
        self.all_occupancies.unset_bit(square);
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
        get_pawn_attack_table   (square, opposite_color(by_color)) .and(self.get_bitboard(by_color, Pawn  )).is_not_empty() ||
        get_knight_attack_table (square)                           .and(self.get_bitboard(by_color, Knight)).is_not_empty() ||
        get_king_attack_table   (square)                           .and(self.get_bitboard(by_color, King  )).is_not_empty() ||
        get_rook_attack_table   (square, self.all_occupancies)     .and(self.get_bitboard(by_color, Rook  )).is_not_empty() ||
        get_bishop_attack_table (square, self.all_occupancies)     .and(self.get_bitboard(by_color, Bishop)).is_not_empty() ||
        get_queen_attack_table  (square, self.all_occupancies)     .and(self.get_bitboard(by_color, Queen )).is_not_empty()
    }

    /// Gets the position of the king of the given color
    #[inline(always)]
    pub fn king_position(&self, color: Color) -> u8 {
        self.get_bitboard(color, King).least_significant()
    }

    #[inline(always)]
    pub fn is_in_check(&self, color: Color) -> bool {
        self.is_square_attacked(self.king_position(color), opposite_color(color))
    }

    /// Get all squares attacked by pieces of this type and color
    pub fn get_attacked(&self, color: Color, piece_type: PieceType) -> Bitboard {
        let mut bb = self.get_bitboard(color, piece_type);
        let mut mask = Bitboard::new_blank();
        while let Some(square) = bb.extract_bit() {
            mask = mask.or(get_attack_table(square, color, piece_type, self.all_occupancies))
        };
        mask
    }
}