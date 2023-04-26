use super::*;
use std::{sync::{atomic::{Ordering::*, AtomicBool}, Arc, Mutex}, thread::{self, JoinHandle}, time::Instant};

const INFINITY : i16 = 30000;

#[derive(Clone)]
pub struct Search {
    is_running: Arc<AtomicBool>,
    is_stopping: Arc<AtomicBool>,
    settings: Arc<Mutex<Settings>>,
    tt: Arc<TranspositionTable>
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

pub fn run_search<const IS_MASTER: bool>(context: &mut SearchContext, is_printing: bool) -> SearchResult {
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
    for depth in 1..=(context.search_meta.max_depth) {
        negamax(&pos, -INFINITY, INFINITY, depth, 0, context);

        if context.search.is_stopping() {
            break;
        }
    }

    //negamax(&pos, -INFINITY, INFINITY, context.search_meta.max_depth, 0, context);

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
        tt_hits: context.tt_hits,
        time
    }
}

fn negamax(pos: &Position, mut alpha: i16, mut beta: i16, depth: u8, ply: u8, context: &mut SearchContext) -> i16 {
    if context.search.is_stopping() { // Cancel search
        return beta
    }

    if depth == 0 {
        return pos.evaluate();
        // Quiescence search
    };

    context.nodes += 1;

    let mut best_move = None;
    
    if let Some(entry) = context.search.tt.probe(pos.zobrist_hash) {
        if entry.depth() >= depth {
            match entry.hash_flag() {
                HashFlag::Exact => return entry.score(),
                HashFlag::LowerBound => alpha = alpha.max(entry.score()),
                HashFlag::UpperBound => beta = beta.min(entry.score()),
            }
            if alpha >= beta {
                return entry.score()
            }
        }
        best_move = Some(entry.best_move());

        context.tt_hits += 1;
    }

    context.pv_table.init_ply(ply);
    let mut hash_flag = HashFlag::UpperBound;

    let mut move_list = pos.generate_moves().sort(pos, context, best_move);

    while let Some(m) = move_list.pop_best() {
        let mut new_pos = *pos;
        new_pos.make_move(m);

        let score = -negamax(&new_pos, -beta, -alpha, depth - 1, ply + 1, context);

        if score > alpha {
            best_move = Some(m);
            context.pv_table.insert_pv_node(m, ply);

            if score >= beta {
                context.search.tt.record(pos.zobrist_hash, best_move, depth, beta, HashFlag::LowerBound);
                return beta;
            }

            hash_flag = HashFlag::Exact;

            alpha = score;
        }
    }
    
    context.search.tt.record(pos.zobrist_hash, best_move, depth, alpha, hash_flag);
    
    alpha
}