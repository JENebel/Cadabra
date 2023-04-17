use std::{sync::{mpsc::{Receiver, Sender, channel}, Arc, atomic::{AtomicBool, Ordering}, Mutex}, thread::{self, JoinHandle}};
use super::*;
use std::sync::atomic::Ordering::*;

#[derive(Clone)]
pub struct Search {
    is_running: Arc<AtomicBool>,
    is_stopping: Arc<AtomicBool>,
    settings: Arc<Mutex<Settings>>
}

impl Search {
    pub fn new(settings: Settings) -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            is_stopping: Arc::new(AtomicBool::new(false)),
            settings: Arc::new(Mutex::new(settings)),
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
                start_search(&mut context);
                context.pv_table.best_move()
            });

            workers.push(h);
        }

        // Wait for workers to terminate
        for (i, w) in workers.into_iter().enumerate() {
            w.join().unwrap();
            if i == 0 {
                self.is_running.store(false, Relaxed)
            }
        }
    }

    pub fn stop(&self) {
        if self.is_running.load(Relaxed) {
            self.is_running.store(false, Relaxed)
        } else {
            println!("Can't stop search, as there is no search running")
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Relaxed)
    }

    /// Resets the transposition table etc.
    pub fn new_game(&self) {
        todo!()
    }
}

/// The arguments provided in go command
#[derive(Clone)]
pub struct SearchMeta {
    depth: u8,
}

impl SearchMeta {
    pub fn new(depth: u8) -> Self {
        Self { depth }
    }
}

/// Messages sent from main to search thread
pub enum SearchMessage {
    Stop,
    PonderHit,
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

fn start_search(context: &mut SearchContext) {

}

fn negamax(pos: &Position, mut alpha: i32, beta: i32, depth: u8, ply: u8, pv_table: &mut PVTable) -> i32 {
    if depth == 0 {
        return pos.evaluate()
    };

    pv_table.pv_lengths[ply as usize] = ply as usize;

    let move_list = pos.generate_moves();

    for m in move_list {
        let mut new_pos = pos.clone();

        new_pos.make_move(m);

        let score = -negamax(&new_pos, -beta, -alpha, depth - 1, ply + 1, pv_table);

        if score > alpha {
            pv_table.insert_pv_node(m, ply);

            if score >= beta {
                return beta;
            }

            alpha = score;
        }
    }

    alpha
}