use super::*;

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
    pub fn score_move(&self, pos: &Position, context: &mut SearchContext) -> i16 {
        /*if envir.score_pv {
            if envir.pv_table[0][envir.ply as usize] == cmove {
                envir.score_pv = false;
                return 20000;
            }
        }*/

        let _ = context.clone();

        if self.is_enpassant() {
            return MVV_LVA[1][1]
        }
    
        let src = self.src();
        let dst = self.dst();
        //Captures
        if self.is_capture() {
            let src_piece = pos.piece_type_at(src);
            let dst_piece = pos.piece_type_at(dst);
    
            MVV_LVA[src_piece.index(Color::White)][dst_piece.index(Color::White)] + 5000
        }
    
        //Quiet moves
        else {
            0
            /*if envir.killer_moves[0][envir.ply as usize] == Some(cmove) {
                9000
            } else if envir.killer_moves[1][envir.ply as usize] == Some(cmove) {
                8000
            }
            else {
                envir.history_moves[cmove.piece() as usize][to_sq as usize]
            }*/
        }
    }
}