use super::*;

const PV_MOVE_SCORE: i16 = 30000;
const BASE_CAPTURE_SCORE: i16 = 29000;
const BASE_KILLER_SCORE: i16 = 28500;

///[attacker][victim]
pub const MVV_LVA: [[i16; 6]; 6] = [
    [105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600],
];

impl Move {
    #[inline(always)]
    pub fn score_move(&self, pos: &Position, context: &mut SearchContext, best_move: Option<Move>, ply: u8) -> i16 {
        if let Some(moove) = best_move {
            if *self == moove {
                return PV_MOVE_SCORE
            }
        }

        // Special case for enpassant
        if self.is_enpassant() {
            return MVV_LVA[0][0] // Pawn x Pawn
        }
    
        let src = self.src();
        let dst = self.dst();
        
        // Captures
        if self.is_capture() {
            let src_piece = pos.piece_type_at(src);
            let dst_piece = pos.piece_type_at(dst);
    
            return MVV_LVA[src_piece.index(Color::White)][dst_piece.index(Color::White)] + BASE_CAPTURE_SCORE
        }

        // Killer moves
        for i in 0..KILLER_MOVE_COUNT {
            if Some(*self) == context.killer_moves[i][ply as usize] {
                return BASE_KILLER_SCORE - i as i16
            }
        }

        let (color, piece) = pos.piece_at(src);
        context.history_moves[piece.index(color)][dst as usize].min(1000)
    }
}