use super::*;
use std::{sync::{atomic::{Ordering::*, AtomicBool}, Arc, Mutex}, thread::{self, JoinHandle}, time::Instant, ops::Add, iter::Sum};

//const INFINITY : i16 = 15000;

#[derive(Clone)]
pub struct Search {
    is_running: Arc<AtomicBool>,
    is_stopping: Arc<AtomicBool>,
    settings: Arc<Mutex<Settings>>,
    // TT here
}

impl Search {
    pub fn new(settings: Settings) -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            is_stopping: Arc::new(AtomicBool::new(false)),
            settings: Arc::new(Mutex::new(settings)),
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

/// The arguments provided in go command
#[derive(Copy, Clone)]
pub struct SearchMeta {
    max_depth: u8,
}

impl SearchMeta {
    pub fn new(max_depth: u8) -> Self {
        assert!(max_depth < MAX_PLY as u8, "Depth must be less than {MAX_PLY}. Was {max_depth}");
        Self { max_depth }
    }
}

#[derive(Clone)]
pub struct SearchContext {
    search: Search,
    search_meta: SearchMeta,
    pos: Position,
    pv_table: PVTable,

    nodes: u128,
}

impl SearchContext {
    pub fn new(search: Search, search_meta: SearchMeta, pos: Position) -> Self {
        Self {
            search,
            search_meta,
            pos,
            pv_table: PVTable::new(),
            nodes: 0
        }
    }
}

pub struct SearchResult {
    pub nodes: u128,
    pub time: u128, // millis
}

impl Add<Self> for SearchResult {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            nodes: self.nodes + rhs.nodes,
            time: self.time
        }
    }
}

impl Sum for SearchResult {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter().reduce(|acc, res| acc + res).unwrap()
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

    negamax(&pos, -15000, 15000, context.search_meta.max_depth, 0, context);

    let time = before.elapsed().as_millis();

    // Stop helper threads
    if IS_MASTER {
        context.search.stop();

        info!("bestmove {}", context.pv_table.best_move().unwrap());
        info!("time {time} ms");
        info!("nodes {}", context.nodes);

        // If ponder enabled, start pondering
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

    context.nodes += 1;

    if ply > depth {
        return pos.evaluate()
    };

    context.pv_table.init_ply(ply);

    let mut move_list = pos.generate_moves().sort(pos, context);

    while let Some(m) = move_list.pop_best() {
        let mut new_pos = pos.clone();
        new_pos.make_move(m);

        let score = -negamax(&new_pos, -beta, -alpha, depth, ply + 1, context);

        if score > alpha {
            context.pv_table.insert_pv_node(m, ply);

            if score >= beta {
                return beta;
            }

            alpha = score;
        }
    }

    alpha
}