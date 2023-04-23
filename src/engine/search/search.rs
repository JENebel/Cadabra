use super::*;
use std::{sync::{atomic::{Ordering::*, AtomicBool}, Arc, Mutex}, thread::{self, JoinHandle}, time::Instant};

const INFINITY : i16 = 30000;

#[derive(Clone)]
pub struct Search {
    is_running: Arc<AtomicBool>,
    is_stopping: Arc<AtomicBool>,
    settings: Arc<Mutex<Settings>>,
    tt: Arc<Mutex<TranspositionTable>>
}

impl Search {
    pub fn new(settings: Settings) -> Self {
        let tt = TranspositionTable::new(settings.transposition_table_mb);
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            is_stopping: Arc::new(AtomicBool::new(false)),
            settings: Arc::new(Mutex::new(settings)),
            tt: Arc::new(Mutex::new(tt)),
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
                let mut context = SearchContext::new(search, meta, pos);
                if t == 0 {       
                    run_search::<true>(&mut context, print)
                } else {       
                    run_search::<false>(&mut context, print)
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

    /// Resets the transposition table etc.
    pub fn new_game(&self) {
        
    }
}

fn run_search<const IS_MASTER: bool>(context: &mut SearchContext, is_printing: bool) -> SearchResult {
    macro_rules! info {
        ($($msg: tt)*) => {
            if is_printing {
                println!($($msg)*)
            }
        };
    }

    let pos = context.pos;

    let before = Instant::now();

    // Iterative deepening loop
    /*for ply in 1..=(context.search_meta.max_depth.min(MAX_PLY as u8 - 1)) {
        negamax(&pos, -15000, 15000, ply, 0, context);

        if context.search.is_stopping() {
            break;
        }
    }*/

    negamax(&pos, -INFINITY, INFINITY, context.search_meta.max_depth, 0, context);

    let time = before.elapsed().as_millis();

    // Stop helper threads
    if IS_MASTER {
        context.search.stop();

        info!("info time {time} nodes {}", context.nodes);
        match context.pv_table.best_move() {
            Some(m) => info!("bestmove {m}"),
            None => info!("bestmove error"),
        }
    };

    SearchResult {
        nodes: context.nodes,
        time
    }
}

fn negamax(pos: &Position, mut alpha: i16, beta: i16, depth: u8, ply: u8, context: &mut SearchContext) -> i16 {
    if context.search.is_stopping() { // Cancel search
        return beta
    }

    // Quiescence search

    context.nodes += 1;

    let is_pv_node = (beta - alpha) > 1;

    if ply != 0 && !is_pv_node {
        match context.search.tt.lock().unwrap().probe(pos.zobrist_hash, depth, alpha, beta) {
            Some(score) => {
                // TT hit
                return score;
            },
            None => ()
        }
    }

    if ply >= depth {
        return pos.evaluate()
    };

    context.pv_table.init_ply(ply);

    let mut move_list = pos.generate_moves().sort(pos, context);

    let mut pv_move = None;

    while let Some(m) = move_list.pop_best() {
        let mut new_pos = pos.clone();
        new_pos.make_move(m);

        let score = -negamax(&new_pos, -beta, -alpha, depth, ply + 1, context);

        if score > alpha {
            pv_move = Some(m);
            context.pv_table.insert_pv_node(m, ply);

            if score >= beta {
                context.search.tt.lock().unwrap().record(pos.zobrist_hash, m, depth, beta, NodeType::Beta);

                return beta;
            }

            alpha = score;
        }
    }
    //envir.transposition_table.record(game.zobrist_hash, temp_alpha, depth, hash_flag, envir.ply);
    
    match pv_move {
        Some(m) => context.search.tt.lock().unwrap().record(pos.zobrist_hash, m, depth, beta, NodeType::Exact),
        None => context.search.tt.lock().unwrap().record_alpha(pos.zobrist_hash, depth, alpha),
    }
    

    alpha
}