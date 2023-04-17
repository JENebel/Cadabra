use super::*;
use std::{sync::{atomic::{Ordering::*, AtomicBool}, Arc, Mutex}, thread::{JoinHandle, self}};

#[derive(Clone)]
pub struct Search {
    is_running: Arc<AtomicBool>,
    is_stopping: Arc<AtomicBool>,
    settings: Arc<Mutex<Settings>>,
    worker_handles: Arc<Mutex<Option<Vec<JoinHandle<()>>>>>,
    // TT here
}

impl Search {
    pub fn new(settings: Settings) -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            is_stopping: Arc::new(AtomicBool::new(false)),
            settings: Arc::new(Mutex::new(settings)),
            worker_handles: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self, pos: Position, meta: SearchMeta) {
        // Spawn worker threads
        let mut workers = Vec::new();
        for t in 0..self.settings.lock().unwrap().threads {
            let search = self.clone();
            let meta = meta.clone();
            let pos = pos.clone();

            let h = thread::spawn(move || {
                let mut context = SearchContext::new(search, meta, pos);
                if t == 0 {       
                    run_search::<true>(&mut context);
                } else {       
                    run_search::<false>(&mut context);
                }
            });

            workers.push(h);
        }

        *self.worker_handles.lock().unwrap() = Some(workers)
    }

    pub fn stop(&self, skip_master: bool) {
        if self.is_running.load(Relaxed) {
            self.is_stopping.store(true, Relaxed);

            // Wait for all threads to terminate
            let handles = self.worker_handles.lock().unwrap().take().unwrap();
            for w in handles.into_iter().skip(if skip_master {1} else {0}) {
                w.join().unwrap();
            }

            self.is_stopping.store(false, Relaxed);
            self.is_running.store(false, Relaxed);
        } else {
            println!("Can't stop search, as there is no search running")
        }

        *self.worker_handles.lock().unwrap() = None;
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Relaxed)
    }

    pub fn is_stopping(&self) -> bool {
        self.is_stopping.load(Relaxed)
    }

    /// Resets the transposition table etc.
    pub fn new_game(&self) {
        todo!()
    }
}

/// The arguments provided in go command
#[derive(Clone)]
pub struct SearchMeta {
    max_depth: u8,
}

impl SearchMeta {
    pub fn new(max_depth: u8) -> Self {
        Self { max_depth }
    }
}

#[derive(Clone)]
pub struct SearchContext {
    search: Search,
    search_meta: SearchMeta,
    pos: Position,
    pv_table: PVTable,
}

impl SearchContext {
    pub fn new(search: Search, search_meta: SearchMeta, pos: Position) -> Self {
        Self {
            search,
            search_meta,
            pos,
            pv_table: PVTable::new()
        }
    }
}

fn run_search<const IS_MASTER: bool>(context: &mut SearchContext) {
    //negamax(pos, alpha, beta, depth, ply, pv_table);
    for ply in 1..(context.search_meta.max_depth.min(MAX_PLY as u8)) {
        negamax(context.pos, i16::MIN, i16::MAX, ply, 0, context);
        if context.search.is_stopping() {
            break;
        }
    }

    // Stop helper threads
    if IS_MASTER {
        if !context.search.is_stopping() {
            context.search.stop(true);
        }

        println!("bestmove {}", context.pv_table.best_move().unwrap());

        // If ponder enabled, start pondering
    }
}

fn negamax(pos: Position, mut alpha: i16, beta: i16, depth: u8, ply: u8, context: &mut SearchContext) -> i16 {
    if depth == 0 {
        return pos.evaluate()
    };

    context.pv_table.init_ply(ply);

    let move_list = pos.generate_moves();

    for m in move_list {
        let mut new_pos = pos.clone();

        new_pos.make_move(m);

        let score = -negamax(new_pos, -beta, -alpha, depth - 1, ply + 1, context);

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