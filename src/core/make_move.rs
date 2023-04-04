use super::*;
use PieceType::*;
use Color::*;
use Square::*;
use MoveType::*;

impl Position {
    pub fn make_uci_move(&mut self, moove: &str) -> Result<(), String> {
        let m = self.generate_moves().into_iter().find(|m| format!("{m}") == moove);
        if let Some(m) = m {
            self.make_move(m);
            Ok(())
        } else {
            Err(format!("Illegal move: {moove}"))
        }
    }

    #[inline(always)]
    pub fn make_move(&mut self, moove: Move) {
        let color = self.active_color;
        let opp_color = color.opposite();

        let src = moove.src();
        let dst = moove.dst();
        let piece = self.piece_type_at(src);
        let move_type = moove.move_type();

        // Unapply current castling ability zobrist (reapplied after castling)
        self.apply_castling_zobrist();

        // Unapply enpassant zobrist
        self.apply_enpassant_zobrist(self.enpassant_square);
        self.enpassant_square = 0;

        match moove.is_capture() && move_type != Enpassant {
            true => self.capture_move(color, src, dst),
            false => self.move_piece(color, src, dst)
        }

        match move_type {
            Quiet | Capture => (),
            Promotion => {
                self.promote_piece(color, dst, moove.promotion())
            },
            DoublePush => {
                let enp_sq = match color {
                    White => dst + 8,
                    Black => dst - 8,
                };
                self.apply_enpassant_zobrist(enp_sq);
                self.enpassant_square = enp_sq;
            },
            CastleKingSide => {
                match color {
                    White => self.move_piece(White, h1 as u8, f1 as u8),
                    Black => self.move_piece(Black, h8 as u8, f8 as u8),
                }
            },
            CastleQueenSide => {
                match color {
                    White =>  self.move_piece(White, a1 as u8, d1 as u8),
                    Black =>  self.move_piece(Black, a8 as u8, d8 as u8)
                }
            },
            Enpassant => {
                let captured = match color {
                    White => dst + 8,
                    Black => dst - 8,
                };
                self.remove_piece(opp_color, captured);
            },
        }

        // Update castling abililties
        self.castling_ability.update(src, dst);
        self.apply_castling_zobrist();

        // Update half moves counter
        if moove.is_capture() || piece == Pawn || moove.is_promotion() {
            self.half_moves = 0;
        }
        else {
            self.half_moves += 1;
        }

        // Increment full moves

        self.full_moves += color as u16;

        // Switch side
        self.active_color = opp_color;
        self.apply_side_zobrist();
    }
}