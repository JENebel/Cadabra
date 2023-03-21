use super::*;
use MoveType::*;
use PieceType::*;
use Color::*;
use Square::*;

impl Position {
    pub fn make_uci_move(&mut self, moov: &str) -> Result<(), String> {
        let m = self.generate_moves().find(|m| format!("{m}") == moov);
        if let Some(m) = m {
            self.make_move(m);
            Ok(())
        } else {
            Err(format!("Illegal move: {moov}"))
        }
    }

    #[inline(always)]
    pub fn make_move(&mut self, moov: Move) {
        //let move_type = moov.move_type;
        let color = self.active_color;
        let opp_color = color.opposite();

        // Unapply current castling ability zobrist (reapplied after castling)
        self.apply_castling_zobrist();

        if moov.is_capture() {
            // Find the taken piece and remove it
            if self.bb(opp_color, Pawn).get_bit(moov.dst) {
                self.remove_piece(opp_color, Pawn, moov.dst);
                self.apply_piece_zobrist(opp_color, Pawn, moov.dst);
            }
            else if self.bb(opp_color, Knight).get_bit(moov.dst) {
                self.remove_piece(opp_color, Knight, moov.dst);
                self.apply_piece_zobrist(opp_color, Knight, moov.dst);
            }
            else if self.bb(opp_color, Bishop).get_bit(moov.dst) {
                self.remove_piece(opp_color, Bishop, moov.dst);
                self.apply_piece_zobrist(opp_color, Bishop, moov.dst);
            }
            else if self.bb(opp_color, Rook).get_bit(moov.dst) {
                self.remove_piece(opp_color, Rook, moov.dst);
                self.apply_piece_zobrist(opp_color, Rook, moov.dst);
            }
            else if self.bb(opp_color, Queen).get_bit(moov.dst) {
                self.remove_piece(opp_color, Queen, moov.dst);
                self.apply_piece_zobrist(opp_color, Queen, moov.dst);
            }
        }

        if moov.move_type == EnpassantCapture {
            let captured = match color {
                White => moov.dst + 8,
                Black => moov.dst - 8,
            };
            
            self.remove_piece(opp_color, Pawn, captured);
            self.apply_piece_zobrist(opp_color, Pawn, captured);
            self.apply_enpassant_zobrist(self.enpassant_square.least_significant());
        }

        if moov.is_castling() {
            let rook_origin;
            let rook_target;

            match (color, moov.move_type) {
                (White, CastleKingSide) => {
                    rook_origin = h1 as u8;
                    rook_target = f1 as u8;
                },
                (White, CastleQueenSide) => {
                    rook_origin = a1 as u8;
                    rook_target = d1 as u8;
                },
                (Black, CastleKingSide) => {
                    rook_origin = h8 as u8;
                    rook_target = f8 as u8;
                },
                (Black, CastleQueenSide) => {
                    rook_origin = a8 as u8;
                    rook_target = d8 as u8;
                },
                _ => unreachable!() // Only enters with castling move_types
            }

            self.remove_piece(color, Rook, rook_origin);
            self.apply_piece_zobrist(color, Rook, rook_origin);

            self.place_piece(color, Rook, rook_target);
            self.apply_piece_zobrist(color, Rook, rook_target);
        }

        if moov.move_type == DoublePush {
            let enp_sq = match color {
                White => moov.dst + 8,
                Black => moov.dst - 8,
            };

            self.enpassant_square = Bitboard(1 << enp_sq);
            self.apply_enpassant_zobrist(enp_sq)
        }
        else {
            self.enpassant_square = Bitboard::EMPTY
        }

        // Remove piece from source
        self.remove_piece(color, moov.piece, moov.src);
        self.apply_piece_zobrist(color, moov.piece, moov.src);

        if moov.is_promotion() {
            // Place upgraded
            let promo = match moov.move_type {
                Promotion(promo) | CapturePromotion(promo) => promo,
                _ => unreachable!()
            };
            self.place_piece(color, promo, moov.dst);
            self.apply_piece_zobrist(color, promo, moov.dst);
        } else {
            // Normally move piece to target square
            self.place_piece(color, moov.piece, moov.dst);
            self.apply_piece_zobrist(color, moov.piece, moov.dst);
        }

        //Update castling abililties
        self.castling_ability.update(moov.src, moov.dst);
        self.apply_castling_zobrist();

        // Update half moves counter
        if moov.is_capture() || moov.piece == Pawn {
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
        self.apply_side_zobrist();
    }
}