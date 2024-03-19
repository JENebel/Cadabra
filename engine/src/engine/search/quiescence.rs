use crate::{Position, SearchContext, Evaluator, engine::moove::Move};

#[inline(always)]
pub fn quiescence(pos: &Position, mut alpha: i16, beta: i16, ply: u8, context: &mut SearchContext, evaluator: impl Evaluator + Copy) -> i16 {
    context.nodes += 1;

    // Evaluate position immediately
    let eval = pos.evaluate(evaluator);

    let in_check = pos.is_in_check();

    if eval > alpha && !in_check {
        alpha = eval;
    }

    if eval >= beta {
        return beta
    }

    // Generate moves
    let mut move_list = pos.generate_moves().sort(pos, context, Move::NULL, ply);

    // Loop through all captures
    while let Some(moove) = move_list.pop_best() {
        if !in_check && (!(moove.is_capture() || moove.is_enpassant())) {
            continue
        }

        let mut copy = pos.clone();
        copy.make_move(moove);
        
        let score = -quiescence(&copy, -beta, -alpha, ply + 1, context, evaluator);

        // Alpha cutoff
        if score > alpha {
            alpha = score;

            // Beta cutoff
            if score >= beta {
                return beta;
            }
        }
    }

    alpha
}