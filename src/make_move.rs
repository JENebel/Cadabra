use super::*;
use MoveType::*;
use PieceType::*;
use Color::*;

pub const CASTLING_RIGHTS: [u8; 64] = [
    7, 15, 15, 15,  3, 15, 15, 11,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   13, 15, 15, 15, 12, 15, 15, 14
];

macro_rules! make_move_match_move_type {
    ($pos: expr, $is_white: expr, $move_to_make: expr) => {
        match $move_to_make.move_type {
            Quiet =>                                $pos.make_move_internal::<$is_white, 0>($move_to_make),
            Capture =>                              $pos.make_move_internal::<$is_white, 1>($move_to_make),
            Promotion(PieceType::Queen) =>          $pos.make_move_internal::<$is_white, 2>($move_to_make),
            Promotion(PieceType::Rook) =>           $pos.make_move_internal::<$is_white, 3>($move_to_make),
            Promotion(PieceType::Bishop) =>         $pos.make_move_internal::<$is_white, 4>($move_to_make),
            Promotion(PieceType::Knight) =>         $pos.make_move_internal::<$is_white, 5>($move_to_make),
            CapturePromotion(PieceType::Queen) =>   $pos.make_move_internal::<$is_white, 6>($move_to_make),
            CapturePromotion(PieceType::Rook) =>    $pos.make_move_internal::<$is_white, 7>($move_to_make),
            CapturePromotion(PieceType::Bishop) =>  $pos.make_move_internal::<$is_white, 8>($move_to_make),
            CapturePromotion(PieceType::Knight) =>  $pos.make_move_internal::<$is_white, 9>($move_to_make),
            CastleKingSide =>                       $pos.make_move_internal::<$is_white, 10>($move_to_make),
            CastleQueenSide =>                      $pos.make_move_internal::<$is_white, 11>($move_to_make),
            DoublePush =>                           $pos.make_move_internal::<$is_white, 12>($move_to_make),
            EnpassantCapture =>                     $pos.make_move_internal::<$is_white, 13>($move_to_make),
            _ => unreachable!(),
        }
    };
}
macro_rules! make_move {
    ($pos: expr, $move_to_make: expr) => {
        match $pos.active_color {
            White => make_move_match_move_type!($pos, true, $move_to_make),
            Black => make_move_match_move_type!($pos, false, $move_to_make)
        }
    };
}

impl Position {
    #[inline(always)]
    pub fn make_move(&mut self, move_to_make: Move) {
        make_move!(self, move_to_make)
    }

    #[inline(always)]
    fn make_move_internal<const IS_WHITE: bool, const MOVE_TYPE: u16>(&mut self, cmove: Move) {
        let move_type = MoveType::from(MOVE_TYPE);
        let color = if IS_WHITE { Color::White } else { Color::Black };
        let opp_color = opposite_color(color);

        // Reset zobrist hashes
        self.zobrist_hash ^= CASTLE_KEYS[self.castling_ability as usize];
        if move_type == EnpassantCapture {
            self.zobrist_hash ^= ENPASSANT_KEYS[unsafe { self.enpassant_square.unwrap_unchecked() } as usize]
        }

        if move_type.is_capture() {
            // Find the taken piece and remove it
            if self.bb(opp_color, Pawn).get_bit(cmove.to_sq) {
                self.remove_piece(opp_color, Pawn, cmove.to_sq);
                self.zobrist_hash ^= PIECE_KEYS[Self::get_bitboard_index(opp_color, Pawn)][cmove.to_sq as usize];
            }
            else if self.bb(opp_color, Knight).get_bit(cmove.to_sq) {
                self.remove_piece(opp_color, Knight, cmove.to_sq);
                self.zobrist_hash ^= PIECE_KEYS[Self::get_bitboard_index(opp_color, Knight)][cmove.to_sq as usize];
            }
            else if self.bb(opp_color, Bishop).get_bit(cmove.to_sq) {
                self.remove_piece(opp_color, Bishop, cmove.to_sq);
                self.zobrist_hash ^= PIECE_KEYS[Self::get_bitboard_index(opp_color, Bishop)][cmove.to_sq as usize];
            }
            else if self.bb(opp_color, Rook).get_bit(cmove.to_sq) {
                self.remove_piece(opp_color, Rook, cmove.to_sq);
                self.zobrist_hash ^= PIECE_KEYS[Self::get_bitboard_index(opp_color, Rook)][cmove.to_sq as usize];
            }
            else if self.bb(opp_color, Queen).get_bit(cmove.to_sq) {
                self.remove_piece(opp_color, Queen, cmove.to_sq);
                self.zobrist_hash ^= PIECE_KEYS[Self::get_bitboard_index(opp_color, Queen)][cmove.to_sq as usize];
            }
        }

        if move_type == EnpassantCapture {
            let captured = match color {
                White => cmove.to_sq + 8,
                Black => cmove.to_sq - 8,
            };
            
            self.remove_piece(opp_color, Pawn, captured);
            self.zobrist_hash ^= PIECE_KEYS[Self::get_bitboard_index(opp_color, Pawn)][captured as usize];
        }

        if move_type.is_promotion() {
            let pawn = Self::get_bitboard_index(color, Pawn);
            self.bitboards[pawn].unset_bit(cmove.to_sq);
            self.zobrist_hash ^= PIECE_KEYS[pawn][cmove.to_sq as usize];

            let queen = Self::get_bitboard_index(color, Queen);
            self.bitboards[queen].set_bit(cmove.to_sq);
            self.zobrist_hash ^= PIECE_KEYS[queen][cmove.to_sq as usize];
        }

        if move_type.is_castling() {
            let rook_origin;
            let rook_target;

            match (color, move_type) {
                (White, CastleKingSide) => {
                    rook_origin = Square::h1 as u8;
                    rook_target = Square::f1 as u8;
                },
                (White, CastleQueenSide) => {
                    rook_origin = Square::a1 as u8;
                    rook_target = Square::d1 as u8;
                },
                (Black, CastleKingSide) => {
                    rook_origin = Square::h8 as u8;
                    rook_target = Square::f8 as u8;
                },
                (Black, CastleQueenSide) => {
                    rook_origin = Square::a8 as u8;
                    rook_target = Square::d8 as u8;
                },
                _ => unreachable!() // Only enters with castling move_types
            }

            let rook_index = Self::get_bitboard_index(color, Rook);

            self.remove_piece(color, Rook, rook_origin);
            self.zobrist_hash ^= PIECE_KEYS[rook_index][rook_origin as usize];

            self.place_piece(color, Rook, rook_target);
            self.zobrist_hash ^= PIECE_KEYS[rook_index][rook_target as usize];
        }

        if move_type == DoublePush {
            let enp_sq = match color {
                White => Square::from(cmove.to_sq + 8),
                Black => Square::from(cmove.to_sq - 8),
            };

            self.enpassant_square = Some(enp_sq);

            self.zobrist_hash ^= ENPASSANT_KEYS[enp_sq as usize];
        }
        else {
            self.enpassant_square = None
        }

        // Move the piece
        self.remove_piece(color, cmove.piece, cmove.from_sq);
        self.zobrist_hash ^= PIECE_KEYS[cmove.piece as usize][cmove.from_sq as usize];
        self.place_piece(color, cmove.piece, cmove.to_sq);
        self.zobrist_hash ^= PIECE_KEYS[cmove.piece as usize][cmove.to_sq as usize];

        //Update castling abililties
        self.castling_ability &= CASTLING_RIGHTS[cmove.to_sq as usize] & CASTLING_RIGHTS[cmove.from_sq as usize];
        self.zobrist_hash ^= CASTLE_KEYS[self.castling_ability as usize];

        // Update half moves counter
        if move_type.is_capture() || cmove.piece == Pawn {
            self.half_moves = 0;
        }
        else {
            self.half_moves += 1;
        }

        //increment full moves
        if color.is_black() {
            self.full_moves += 1;
        }

        // Switch side
        self.active_color = opp_color;
        self.zobrist_hash ^= SIDE_KEY;
    }
}