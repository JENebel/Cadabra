use super::*;
use std::{sync::{atomic::{Ordering::*, AtomicBool}, Arc, Mutex}, thread::{self, JoinHandle}, time::Instant};

#[derive(Clone)]
pub struct Search {
    is_running: Arc<AtomicBool>,
    is_stopping: Arc<AtomicBool>,
    settings: Arc<Mutex<Settings>>,
    tt: Arc<TranspositionTable>,
}

impl Search {
    pub fn new(settings: Settings) -> Self {
        let tt = TranspositionTable::new(settings.transposition_table_mb);
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            is_stopping: Arc::new(AtomicBool::new(false)),
            settings: Arc::new(Mutex::new(settings)),
            tt: Arc::new(tt),
        }
    }

    pub fn update_settings(&mut self, new_settings: Settings) {
        // New tt if size changed
        if new_settings.transposition_table_mb != self.settings.lock().unwrap().transposition_table_mb {
            self.tt = Arc::new(TranspositionTable::new(new_settings.transposition_table_mb))
        }

        *self.settings.lock().unwrap() = new_settings;
    }

    pub fn clear_hash(&mut self, new_settings: Settings) {
        self.tt = Arc::new(TranspositionTable::new(new_settings.transposition_table_mb))
    }

    /// Returns the running time
    pub fn start(&self, pos: Position, meta: SearchArgs, print: bool) -> SearchStats {
        self.is_running.store(true, Relaxed);

        // Spawn worker threads
        let mut workers = Vec::new();
        for t in 0..self.settings.lock().unwrap().threads {
            let search = self.clone();
            let meta = meta.clone();
            let pos = pos.clone();

            let h: JoinHandle<SearchStats> = thread::spawn(move || {
                let mut context = SearchContext::new(search, meta, pos, Instant::now(), print);
                if t == 0 {
                    run_search::<true>(&mut context, t)
                } else {       
                    run_search::<false>(&mut context, t)
                }
            });

            workers.push(h);
        }

        // Wait for all threads to terminate and combine results
        let result: SearchStats = workers.into_iter().map(|w| w.join().unwrap()).sum();

        self.is_stopping.store(false, Relaxed);
        self.is_running.store(false, Relaxed);

        result
    }

    pub fn stop(&self) {
        self.is_stopping.store(true, Relaxed);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Relaxed)
    }

    pub fn is_stopping(&self) -> bool {
        self.is_stopping.load(Relaxed)
    }
}

macro_rules! info {
    ($context:expr, $($msg: tt)*) => {
        if IS_MASTER && $context.is_printing {
            println!($($msg)*)
        }
    };
}

fn score_str(score: i16) -> String {
    if score >= -MATE_VALUE && score < -MATE_BOUND {
        format!("mate {}", -(score + MATE_VALUE) / 2 - 1)
    } else if score <= MATE_VALUE && score > MATE_BOUND {
        format!("mate {}", (MATE_VALUE - score) / 2 + 1)
    } else {
        format!("cp {}", score)
    }
}

pub fn run_search<const IS_MASTER: bool>(context: &mut SearchContext, thread_id: u8) -> SearchStats {
    let pos = context.pos;

    let mut best_move = Option::None;
    let (mut alpha, mut beta) = (-INFINITY, INFINITY);

    // Iterative deepening loop
    for depth in (thread_id % 4 + 1)..=(context.search_meta.max_depth) {
        // Run initial search with narrow search (Except first time)
        let mut score = negamax::<IS_MASTER>(&pos, alpha, beta, depth, 0, context);

        // Widening aspiration window
        let mut alpha_mult = 1;
        let mut beta_mult = 1;
        loop {
            if context.search.is_stopping() {
                break;
            }

            if score <= alpha {
                // Widen window alpha side
                alpha_mult *= ASPIRATION_WINDOW_MULT;
                alpha = (score as i32 - alpha_mult as i32 * ASPIRATION_WINDOW as i32).min(-INFINITY as i32) as i16;
                score = negamax::<IS_MASTER>(&pos, alpha, beta, depth, 0, context);
            } else if score >= beta {
                // Widen window beta side
                beta_mult *= ASPIRATION_WINDOW_MULT;
                beta = (score as i32 + beta_mult as i32 * ASPIRATION_WINDOW as i32).max(INFINITY as i32) as i16;
                score = negamax::<IS_MASTER>(&pos, alpha, beta, depth, 0, context);
            } else {
                // Succcessful search
                // Reset window for next iteration
                (alpha, beta) = (score - ASPIRATION_WINDOW, score + ASPIRATION_WINDOW);
                break;
            }
        }

        if context.search.is_stopping() {
            break;
        }
        
        best_move = context.pv_table.best_move();

        let time = context.start_time.elapsed().as_millis();
        info!(context, "info score {} depth {depth} nodes {} time {} pv {}", score_str(score), context.nodes + context.qui_nodes, time, context.pv_table);
    }

    // Stop helper threads
    if IS_MASTER {
        context.search.stop();
        context.search.is_running.store(false, Relaxed);

        info!(context, "bestmove {}", best_move.unwrap())
    };

    SearchStats {
        nodes: context.nodes + context.qui_nodes,
        tt_hits: context.tt_hits,
        time: context.start_time.elapsed().as_millis()
    }
}

fn negamax<const IS_MASTER: bool>(pos: &Position, mut alpha: i16, mut beta: i16, mut depth: u8, ply: u8, context: &mut SearchContext) -> i16 {
    let mut best_move = None;

    // Detect 50 move rule and 3 fold repetition stalemate
    if pos.half_moves == 100 || pos.rep_table.is_in_3_fold_rep(pos) {
        return 0
    }
    
    let is_pv = (beta as i32 - alpha as i32) > 1;

    // Probe transposition table
    if let Some(entry) = context.search.tt.probe(pos.zobrist_hash) {
        // Ply > 0 or we risk not knowing the move. !is_pv, or we get weird stuff happening
        if !is_pv && ply > 0 {
            // Adjust mating score
            let score = entry.score();
            let score = if score < -MATE_BOUND {
                score + ply as i16
            } else if score > MATE_BOUND {
                score - ply as i16
            } else {
                score
            };

            if entry.depth() >= depth {
                match entry.hash_flag() {
                    HashFlag::Exact => return score,
                    HashFlag::LowerBound => alpha = alpha.max(score),
                    HashFlag::UpperBound => beta = beta.min(score),
                }
                if alpha >= beta {
                    return score
                }
            }
        }
     
        best_move = Some(entry.best_move());

        context.tt_hits += 1;
    }

    context.pv_table.pv_lengths[ply as usize] = ply as usize;

    if ply == MAX_PLY as u8 - 1 {
        return pos.evaluate();
    }

    // Run quiescence search if the desired depth is reached
    if depth == 0 {
        return quiescence::<IS_MASTER>(pos, alpha, beta, ply, context);
    };

    // Stop search if time is exceeded. Only rarely check to reduce performance impact
    if context.nodes & 0b11111111111 == 0 {
        if IS_MASTER && context.exceeded_time_target() && !context.search.is_stopping() {
            context.search.stop();
            return 0
        } else if context.search.is_stopping() { // Cancel search
            return 0
        }
    }

    context.nodes += 1;

    // Initialize TT entry hashflag
    let mut hash_flag = HashFlag::UpperBound;

    // Check if in check
    let is_in_check = pos.is_in_check();

    if is_in_check { depth += 1 };

    // NULL MOVE PRUNING
    /*let only_pawns_left = pos.bb(pos.active_color, PieceType::Pawn).count_bits() + 1 == pos.color_bb(pos.active_color).count_bits();
    if !is_pv && !is_in_check && depth >= 3{
        let mut new_pos = *pos;
        new_pos.make_null_move();

        let score = -negamax::<IS_MASTER>(&new_pos, -beta, -beta + 1, depth - 3, ply + 1, context);

        if score >= beta {
            return score
        }
    }*/

    // Generate moves
    let mut move_list = pos.generate_moves().sort(pos, context, best_move, ply);

    // detect mate or stalemate if there are no legal moves
    if move_list.len() == 0 {
        if is_in_check {
            return -MATE_VALUE + ply as i16;
        }
        else {
            return 0;
        }
    }

    let mut moves_searched = 0;
    // Loop through moves
    while let Some(moove) = move_list.pop_best() {
        let mut new_pos = *pos;
        new_pos.make_move(moove);
        let caused_check = new_pos.is_in_check();

        let mut score;
        if moves_searched == 0 {
            // Full search in left-most node
            score = -negamax::<IS_MASTER>(&new_pos, -beta, -alpha, depth - 1, ply + 1, context);
        } else {
            // LATE MOVE REDUCTIONS

            // Determine if LMR should be used
            let reduced = !is_pv 
                && depth >= 3 
                && !moove.is_capture() 
                && !moove.is_promotion() 
                && !is_in_check 
                && !caused_check 
                && moves_searched >= 4;

            if reduced {
                // Reduced null window search


                // Opt 1
                let reduction = if moves_searched < 10 { 1 } else { depth / 3 };

                // Opt 2
                // let reduction = if moves_searched < 10 { 1 } else { 2 };

                // opt 3
                // let reduction = 1;


                score = -negamax::<IS_MASTER>(&new_pos, -alpha - 1, -alpha, depth - 1 - reduction, ply + 1, context);

                if score > alpha {
                    // Full null window search on failure
                    score = -negamax::<IS_MASTER>(&new_pos, -alpha - 1, -alpha, depth - 1, ply + 1, context);

                    if score > alpha {
                        // Full search on failure
                        score = -negamax::<IS_MASTER>(&new_pos, -beta, -alpha, depth - 1, ply + 1, context);
                    }
                }
            } else {
                // Full null window search
                score = -negamax::<IS_MASTER>(&new_pos, -alpha - 1, -alpha, depth - 1, ply + 1, context);

                if score > alpha {
                    // Full search on failure
                    score = -negamax::<IS_MASTER>(&new_pos, -beta, -alpha, depth - 1, ply + 1, context);
                }
            }
        }

        // Alpha cutoff
        if score > alpha {
            best_move = Some(moove);

            // Insert PV node
            context.pv_table.insert_pv_node(moove, ply);

            // Beta cutoff
            if score >= beta {
                // Record killer move
                if !moove.is_capture() {
                    context.insert_killer_move(moove, ply)
                }

                // Record lower bound score in TT
                context.search.tt.record(pos.zobrist_hash, best_move, depth, beta, HashFlag::LowerBound, ply);
                
                // Return early
                return beta;
            }

            // We now have an exact score to store in TT, as it is a PV node
            hash_flag = HashFlag::Exact;

            // Update alpha
            alpha = score;
        }

        moves_searched += 1;
    }
    
    // Record upper bound/exact score in TT depending on if we have a PV node
    context.search.tt.record(pos.zobrist_hash, best_move, depth, alpha, hash_flag, ply);
    
    alpha
}

/// Quiescence search
#[inline(always)]
fn quiescence<const IS_MASTER: bool>(pos: &Position, mut alpha: i16, beta: i16, ply: u8, context: &mut SearchContext) -> i16 {
    context.qui_nodes += 1;

    // Evaluate position immediately
    let eval = pos.evaluate();

    // Dont't go on if reached max ply
    if ply == MAX_PLY as u8 - 1 || pos.half_moves == 100 {
        return eval;
    }

    // Try cutoff
    if eval > alpha {
        alpha = eval;

        if eval >= beta {
            return beta
        }
    }

    // Generate moves
    let moves = pos.generate_moves().sort(pos, context, None, ply);

    // Loop through all captures
    for m in moves.filter(|m| m.is_capture()) {
        let mut copy = pos.clone();
        copy.make_move(m);
        
        let score = -quiescence::<IS_MASTER>(&copy, -beta, -alpha, ply + 1, context);

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