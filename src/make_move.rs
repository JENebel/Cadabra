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

impl Position {
    pub fn make_uci_move(&mut self, mov: &str) -> Result<(), String> {
        let m = self.generate_moves().find(|m| format!("{m}") == mov);
        if let Some(m) = m {
            self.make_move(m);
            Ok(())
        } else {
            Err(format!("Illegal move: {mov}"))
        }
    }

    #[inline(always)]
    pub fn make_move(&mut self, cmove: Move) {
        let move_type = cmove.move_type;
        let color = self.active_color;
        let opp_color = color.opposite();

        // Reset zobrist hashes
        self.zobrist_hash ^= CASTLE_KEYS[self.castling_ability as usize];
        if move_type == EnpassantCapture {
            self.zobrist_hash ^= ENPASSANT_KEYS[self.enpassant_square.least_significant() as usize]
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
                White => cmove.to_sq + 8,
                Black => cmove.to_sq - 8,
            };

            self.enpassant_square = Bitboard(1 << enp_sq);

            self.zobrist_hash ^= ENPASSANT_KEYS[enp_sq as usize];
        }
        else {
            self.enpassant_square = Bitboard::EMPTY
        }

        // Remove piece from source
        self.remove_piece(color, cmove.piece, cmove.from_sq);
        self.zobrist_hash ^= PIECE_KEYS[cmove.piece as usize][cmove.from_sq as usize];
        

        if move_type.is_promotion() {
            // Place upgraded
            let promo = match move_type {
                Promotion(promo) | CapturePromotion(promo) => promo,
                _ => unreachable!()
            };
            self.place_piece(color, promo, cmove.to_sq);
            self.zobrist_hash ^= PIECE_KEYS[promo as usize][cmove.to_sq as usize];
        } else {
            // Place same piece on destination
            self.place_piece(color, cmove.piece, cmove.to_sq);
            self.zobrist_hash ^= PIECE_KEYS[cmove.piece as usize][cmove.to_sq as usize];
        }

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