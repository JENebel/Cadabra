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

    /// Returns the running time
    pub fn start(&self, pos: Position, meta: SearchMeta, print: bool) -> SearchResult {
        self.is_running.store(true, Relaxed);

        // Spawn worker threads
        let mut workers = Vec::new();
        for t in 0..self.settings.lock().unwrap().threads {
            let search = self.clone();
            let meta = meta.clone();
            let pos = pos.clone();

            let h: JoinHandle<SearchResult> = thread::spawn(move || {
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
        let result: SearchResult = workers.into_iter().map(|w| w.join().unwrap()).sum();

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

pub fn time_left(nano_times: &Vec<u128>, target_millis: u128) -> bool {
    if nano_times.len() < 2 {
        return true
    }

    // Estimate next time
    let mut incs: Vec<f64> = Vec::new();
    for t in nano_times.windows(2).rev().take(3) {
        incs.push((t[1] as f64) / (t[0] as f64));
    }


    let avg: f64 = incs.iter().sum::<f64>() / incs.len() as f64;
    let last = (*nano_times.last().unwrap() / 1000000) as f64;
    let estimate = last + last * (avg as f64);
    //println!("est. {} ms", estimate);
    
    target_millis > estimate as u128
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
        format!("mate {} ", -(score + MATE_VALUE) / 2 - 1)
    } else if score <= MATE_VALUE && score > MATE_BOUND {
        format!("mate {}", (MATE_VALUE - score) / 2 + 1)
    } else {
        format!("cp {}", score)
    }
}

pub fn run_search<const IS_MASTER: bool>(context: &mut SearchContext, thread_id: u8) -> SearchResult {
    let pos = context.pos;

    // Iterative deepening loop
    for depth in (thread_id % 4 + 1)..=(context.search_meta.max_depth) {
        let score = negamax::<IS_MASTER>(&pos, -INFINITY, INFINITY, depth, 0, context);

        let time = context.start_time.elapsed().as_millis();
        info!(context, "info score {} depth {depth} nodes {} time {} pv {}", score_str(score), context.nodes, time, context.pv_table);

        if context.search.is_stopping() || score > MATE_BOUND || score < -MATE_BOUND {
            break;
        }
    }

    //negamax(&pos, -INFINITY, INFINITY, context.search_meta.max_depth, 0, context);

    // Stop helper threads
    if IS_MASTER {
        context.search.stop();
        context.search.is_running.store(false, Relaxed);

        match context.pv_table.best_move() {
            Some(m) => info!(context, "bestmove {m}"),
            None => panic!("No best move!"),
        }
    };

    SearchResult {

        nodes: context.nodes,
        tt_hits: context.tt_hits,
        time: context.start_time.elapsed().as_millis()
    }
}

fn negamax<const IS_MASTER: bool>(pos: &Position, mut alpha: i16, mut beta: i16, depth: u8, ply: u8, context: &mut SearchContext) -> i16 {
    let mut best_move = None;
    
    // Ply > 0 or we risk not knowing the move
    if ply > 0 {
        if let Some(entry) = context.search.tt.probe(pos.zobrist_hash) {
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
            best_move = Some(entry.best_move());
    
            context.tt_hits += 1;
        }
    }

    if depth == 0 {
        return quiescence::<IS_MASTER>(pos, alpha, beta, ply, context);
        //return pos.evaluate();
    };

    if context.nodes & 0b111111111111 == 0 {
        if IS_MASTER && context.exceeded_time_target() && !context.search.is_stopping() {
            context.search.stop();
            return 0
        } else if context.search.is_stopping() { // Cancel search
            return 0
        }
    }
    context.nodes += 1;

    context.pv_table.init_ply(ply);
    let mut hash_flag = HashFlag::UpperBound;

    let is_in_check = pos.is_in_check();

    let mut move_list = pos.generate_moves().sort(pos, context, best_move);

    // Mate & Draw
    if move_list.len() == 0 {
        if is_in_check {
            return -MATE_VALUE + ply as i16;
        }
        else {
            return 0;
        }
    }

    while let Some(m) = move_list.pop_best() {
        let mut new_pos = *pos;
        new_pos.make_move(m);

        let score = -negamax::<IS_MASTER>(&new_pos, -beta, -alpha, depth - 1, ply + 1, context);

        if score > alpha {
            best_move = Some(m);

            context.pv_table.insert_pv_node(m, ply);

            // Beta cutoff
            if score >= beta {
                context.search.tt.record(pos.zobrist_hash, best_move, depth, beta, HashFlag::LowerBound, ply);
                return beta;
            }

            hash_flag = HashFlag::Exact;

            alpha = score;
        }
    }
    
    context.search.tt.record(pos.zobrist_hash, best_move, depth, alpha, hash_flag, ply);
    
    alpha
}

#[inline(always)]
fn quiescence<const IS_MASTER: bool>(pos: &Position, mut alpha: i16, beta: i16, ply: u8, context: &mut SearchContext) -> i16 {
    if context.nodes & 0b111111111111 == 0 {
        if IS_MASTER && context.exceeded_time_target() && !context.search.is_stopping() {
            context.search.stop();
            return 0
        } else if context.search.is_stopping() { // Cancel search
            return 0
        }
    }
    context.nodes += 1;

    let eval = pos.evaluate();

    // Dont't go on if reached max ply
    if ply > MAX_PLY as u8 - 1 || pos.half_moves == 100 {
        return eval;
    }

    if eval > alpha {
        alpha = eval;

        if eval >= beta {
            return beta
        }
    }

    let moves = pos.generate_moves().sort(pos, context, None);

    for m in moves.filter(|m| m.is_capture()) {
        let mut copy = pos.clone();
        copy.make_move(m);
        
        let score = -quiescence::<IS_MASTER>(&copy, -beta, -alpha, ply + 1, context);

        if score > alpha {
            alpha = score;

            if score >= beta {
                return beta;
            }
        }
    }

    alpha
}