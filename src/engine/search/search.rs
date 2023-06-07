use super::*;
use std::{sync::{atomic::{Ordering::*, AtomicBool}, Arc, Mutex}, thread::{self, JoinHandle}, time::Instant};

#[derive(Clone)]
pub struct Search {
    is_running: Arc<AtomicBool>,
    is_stopping: Arc<AtomicBool>,
    settings: Arc<Mutex<Settings>>,
    pub tt: Arc<TranspositionTable>,
    pub generation: Arc<Mutex<u8>>,
}

impl Search {
    pub fn new(settings: Settings) -> Self {
        let tt = TranspositionTable::new(settings.transposition_table_mb);
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            is_stopping: Arc::new(AtomicBool::new(false)),
            settings: Arc::new(Mutex::new(settings)),
            tt: Arc::new(tt),
            generation: Arc::new(Mutex::new(0)),
        }
    }

    pub fn update_settings(&mut self, new_settings: Settings) {
        // New tt if size changed
        if new_settings.transposition_table_mb != self.settings.lock().unwrap().transposition_table_mb {
            self.tt = Arc::new(TranspositionTable::new(new_settings.transposition_table_mb));
        }

        *self.settings.lock().unwrap() = new_settings;
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
        let result: SearchStats = workers.into_iter().map(|w| match w.join() {
            Ok(stats) => stats,
            Err(err) => panic!("Worker thread panicked with error: {:?}", err),
        }).sum();

        self.is_stopping.store(false, Relaxed);
        self.is_running.store(false, Relaxed);

        // Increment generation
        let mut gen = *self.generation.lock().unwrap() + 1;
        if gen >= 64 { gen = 0 }; // Wrap around
        *self.generation.lock().unwrap() = gen;

        result
    }

    pub fn stop(&self) {
        if self.is_running() {
            self.is_stopping.store(true, Release);
        }
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
        let mut alpha_mult: i32 = 1;
        let mut beta_mult = 1;
        loop {
            if context.search.is_stopping() {
                break;
            }

            if score <= alpha {
                // Widen window alpha side
                alpha_mult *= ASPIRATION_WINDOW_MULT;
                alpha = (score as i32 - alpha_mult * ASPIRATION_WINDOW).max(-INFINITY as i32) as i16;
                score = negamax::<IS_MASTER>(&pos, alpha, beta, depth, 0, context);
            } else if score >= beta {
                // Widen window beta side
                beta_mult *= ASPIRATION_WINDOW_MULT;
                beta = (score as i32 + beta_mult * ASPIRATION_WINDOW).min(INFINITY as i32) as i16;
                score = negamax::<IS_MASTER>(&pos, alpha, beta, depth, 0, context);

            } else {
                // Succcessful search
                // Reset window for next iteration
                (alpha, beta) = (score - ASPIRATION_WINDOW as i16, score + ASPIRATION_WINDOW as i16);
                break;
            }
        }

        if context.search.is_stopping() {
            break;
        }
        
        best_move = context.pv_table.best_move();

        let time = context.start_time.elapsed().as_millis();
        info!(context, "info score {} depth {depth} nodes {} time {} pv {}", score_str(score), context.nodes, time, context.pv_table);
    }

    // Stop helper threads
    if IS_MASTER {
        context.search.stop();
        info!(context, "bestmove {}", best_move.unwrap());
    };

    SearchStats {
        nodes: context.nodes,
        tt_hits: context.tt_hits,
        time: context.start_time.elapsed().as_millis()
    }
}

fn negamax<const IS_MASTER: bool>(pos: &Position, mut alpha: i16, mut beta: i16, mut depth: u8, ply: u8, context: &mut SearchContext) -> i16 {
    // Stop search if signalled or time ran out
    if IS_MASTER && context.exceeded_time_target() {
        context.search.stop();
        return 0
    } else if context.search.is_stopping() { // Cancel search
        return 0
    }

    context.nodes += 1;

    // Initialize PV table entry
    context.pv_table.pv_lengths[ply as usize] = ply as usize;

    // Mate distance pruning
    beta = beta.min(MATE_VALUE - ply as i16);
    alpha = alpha.max(-MATE_VALUE + ply as i16);
    if alpha >= beta { 
        return alpha;
    }

    // Detect 50 move rule, 3 fold repetition and insufficient material stalemates
    if pos.half_moves == 100 || pos.rep_table.is_in_3_fold_rep(pos) || pos.is_insufficient_material() {
        return 0
    }

    // Check extension
    let in_check = pos.is_in_check();
    if in_check { depth += 1 };

    // Abort if hard maximum depth is reached
    if ply == MAX_PLY as u8 {
        return pos.evaluate()
    }

    // Run quiescence search if the desired depth is reached
    if depth == 0 {
        context.nodes -= 1; // Adjust node count to avoid double counting
        return quiescence::<IS_MASTER>(pos, alpha, beta, ply, context);
    };
    
    // Check if we are in a PV node
    let is_pv = (beta as i32 - alpha as i32) > 1;

    // Probe transposition table
    let mut tt_move = Move::NULL;
    if let Some(entry) = context.search.tt.probe(pos.zobrist_hash, ply) {
        context.tt_hits += 1;
        tt_move = entry.best_move;
        
        // !is_pv, or we get weird stuff happening
        if !is_pv {
            // Adjust mating score
            if entry.depth >= depth {
                match entry.flag {
                    HashFlag::Exact => return entry.score,
                    HashFlag::LowerBound => alpha = alpha.max(entry.score),
                    HashFlag::UpperBound => beta = beta.min(entry.score),
                }

                if alpha >= beta {
                    return entry.score
                }
            }
        }
    }

    // Initialize TT entry hashflag
    let mut hash_flag = HashFlag::UpperBound;

    // Do a static evaluation for later use
    let static_eval = pos.evaluate();

    // Reverse futility pruning
    let can_futility_prune = !in_check && !is_pv;
    if can_futility_prune {
        if depth == 1 {
            if static_eval - FRONTIER_FUTILITY_MARGIN >= beta {
                return static_eval - FRONTIER_FUTILITY_MARGIN
            }
        } else if depth == 2 {
            if static_eval - PRE_FRONTIER_FUTILITY_MARGIN >= beta {
                return static_eval - PRE_FRONTIER_FUTILITY_MARGIN
            }
        }
    }

    // Null move pruning
    const R: u8 = 2;
    let only_pawns_left = pos.bb(pos.active_color, PieceType::Pawn).pop_count() + 1 == pos.color_bb(pos.active_color).pop_count();
    let can_nmp = !is_pv
        && !in_check 
        && depth >= R + 1
        && !only_pawns_left 
        && static_eval >= beta;

    if can_nmp {
        let mut new_pos = *pos;
        new_pos.make_null_move();

        let score = -negamax::<IS_MASTER>(&new_pos, -beta, -beta + 1, depth - 1 - R, ply + 1, context);

        if score >= beta {
            return beta
        }
    }

    // Generate moves
    let mut move_list = pos.generate_moves().sort(pos, context, tt_move, ply);

    // Main move loop
    let mut moves_searched = 0;
    while let Some(moove) = move_list.pop_best() {
        moves_searched += 1;

        let mut new_pos = *pos;
        new_pos.make_move(moove);
        let caused_check = new_pos.is_in_check();

        let mut score;
        if moves_searched == 0 {
            // Full search in left-most node
            score = -negamax::<IS_MASTER>(&new_pos, -beta, -alpha, depth - 1, ply + 1, context);
        } else {
            // Late move reductions
            // Determine if LMR should be used
            let can_lmr = !is_pv 
                && depth >= 3
                && !moove.is_capture() 
                && !moove.is_promotion() 
                && !in_check 
                && !caused_check 
                && moves_searched >= 4;

            if can_lmr {
                // Reduced null window search
                let reduction = if moves_searched >= 6 { 2 } else { 1 };

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
            tt_move = moove;

            // Insert PV node
            context.pv_table.insert_pv_node(moove, ply);

            // Beta cutoff
            if score >= beta {
                // Record killer move
                if !moove.is_capture() {
                    context.insert_killer_move(moove, ply)
                }

                // Record lower bound score in TT
                context.search.tt.record(pos.zobrist_hash, tt_move, depth, beta, HashFlag::LowerBound, ply, context.tt_age);
                
                // Return early
                return beta;
            }

            // We now have an exact score to store in TT, as it is a PV node
            hash_flag = HashFlag::Exact;

            // Record history move
            if !moove.is_capture() {
                let (color, piece) = pos.piece_at(moove.src());
                context.insert_history_move(moove, (color, piece), depth)
            }

            // Update alpha
            alpha = score;
        }
    }

    // Detect mate or stalemate if there are no legal moves
    if moves_searched == 0 {
        if in_check {
            alpha = -MATE_VALUE + ply as i16;
        }
        else {
            alpha = 0;
        }
    }
    
    // Record upper bound/exact score in TT depending on if we have a PV node
    context.search.tt.record(pos.zobrist_hash, tt_move, depth, alpha, hash_flag, ply, context.tt_age);
    
    alpha
}

/// Quiescence search
#[inline(always)]
fn quiescence<const IS_MASTER: bool>(pos: &Position, mut alpha: i16, beta: i16, ply: u8, context: &mut SearchContext) -> i16 {
    context.nodes += 1;

    // Evaluate position immediately
    let eval = pos.evaluate();

    // Dont't go on if reached max ply
    if ply == MAX_PLY as u8 {
        return eval;
    }

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