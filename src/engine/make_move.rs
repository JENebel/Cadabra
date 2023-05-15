use super::*;
use PieceType::*;
use Color::*;
use Square::*;

impl Position {
    pub fn make_uci_move(&mut self, moove: &str) -> Result<(), String> {
        let m = self.generate_moves().find(|m| format!("{m}") == moove);
        if let Some(m) = m {
            self.make_move(m);
            Ok(())
        } else {
            Err(format!("Illegal move: {moove}"))
        }
    }

    pub fn _make_null_move(&mut self) {
        // Switch side
        self.active_color = self.active_color.opposite();
        self.apply_side_zobrist();

        // Reset enpassant + update hash
        if !self.enpassant_square.is_empty() {
            self.apply_enpassant_zobrist(self.enpassant_square.least_significant());
            self.enpassant_square = Bitboard::EMPTY;
        };
    }

    #[inline(always)]
    pub fn make_move(&mut self, moove: Move) {
        // Let move_type = moov.move_type;
        let color = self.active_color;
        let opp_color = color.opposite();

        let src = moove.src();
        let dst = moove.dst();
        let piece = self.piece_type_at(src);

        // Unapply current castling ability zobrist (reapplied after castling)
        self.apply_castling_zobrist();

        if moove.is_capture() && !moove.is_enpassant() {
            let captured = self.piece_type_at(dst);
            self.remove_piece(opp_color, captured, dst);
            self.apply_piece_zobrist(opp_color, captured, dst);
        }

        if moove.is_enpassant() {
            let captured = match color {
                White => dst + 8,
                Black => dst - 8,
            };
            
            self.remove_piece(opp_color, Pawn, captured);
            self.apply_piece_zobrist(opp_color, Pawn, captured);
            self.apply_enpassant_zobrist(self.enpassant_square.least_significant());
        }

        // Castling KS
        if moove.is_castle_ks() {
            match color {
                White => {
                    self.remove_piece(color, Rook, h1 as u8);
                    self.apply_piece_zobrist(color, Rook, h1 as u8);

                    self.place_piece(color, Rook, f1 as u8);
                    self.apply_piece_zobrist(color, Rook, f1 as u8);
                },
                Black => {
                    self.remove_piece(color, Rook, h8 as u8);
                    self.apply_piece_zobrist(color, Rook, h8 as u8);

                    self.place_piece(color, Rook, f8 as u8);
                    self.apply_piece_zobrist(color, Rook, f8 as u8);
                },
            }
        }

        // Castling QS
        if moove.is_castle_qs() {
            match color {
                White => {
                    self.remove_piece(color, Rook, a1 as u8);
                    self.apply_piece_zobrist(color, Rook, a1 as u8);

                    self.place_piece(color, Rook, d1 as u8);
                    self.apply_piece_zobrist(color, Rook, d1 as u8);
                },
                Black => {
                    self.remove_piece(color, Rook, a8 as u8);
                    self.apply_piece_zobrist(color, Rook, a8 as u8);

                    self.place_piece(color, Rook, d8 as u8);
                    self.apply_piece_zobrist(color, Rook, d8 as u8);
                },
            }
        }

        if moove.is_double_push() {
            let enp_sq = match color {
                White => dst + 8,
                Black => dst - 8,
            };

            self.enpassant_square = Bitboard(1 << enp_sq);
            self.apply_enpassant_zobrist(enp_sq)
        }
        else {
            self.enpassant_square = Bitboard::EMPTY
        }

        if moove.is_promotion() {
            // Place promotion
            self.place_piece(color, moove.promotion(), dst);
            self.apply_piece_zobrist(color, moove.promotion(), dst);

            // Remove pawn from source
            self.remove_piece(color, Pawn, src);
            self.apply_piece_zobrist(color, Pawn, src);
        } else {
            // Normally move piece to target square
            self.place_piece(color, piece, dst);
            self.apply_piece_zobrist(color, piece, dst);

            self.remove_piece(color, piece, src);
            self.apply_piece_zobrist(color, piece, src);
        }

        // Update castling abililties
        self.castling_ability.update(src, dst);
        self.apply_castling_zobrist();

        // Increment full moves
        self.full_moves += color as u16;

        // Switch side
        self.active_color = opp_color;
        self.apply_side_zobrist();

        // Update half moves counter
        if moove.is_capture() || piece == Pawn || moove.is_promotion() {
            self.half_moves = 0;
            self.rep_table.clear();
        }
        else {
            self.half_moves += 1;
            self.rep_table.push(self.zobrist_hash)
        };
    }
}