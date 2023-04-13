use std::{sync::{mpsc::{Receiver, Sender, channel}, RwLock, Arc, atomic::AtomicBool, Mutex}, thread};
use super::*;
use std::sync::atomic::Ordering::*;

#[derive(Clone)]
pub struct Search {
    is_running: Arc<AtomicBool>,
    sender: Arc<Mutex<Option<Sender<SearchMessage>>>>,
    settings: Arc<Mutex<Settings>>,
}

impl Search {
    pub fn new(settings: Settings) -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            sender: Arc::new(Mutex::new(None)),
            settings: Arc::new(Mutex::new(settings)),
        }
    }

    pub fn start(&self, info: SearchInfo) {
        let senders: Vec<Sender<SearchMessage>> = Vec::new();

        // Spawn worker threads
        for t in 0..self.settings.lock().unwrap().threads {
            let curr_search_clone = self.clone();
            let (sender, receiver) = channel();
            senders.push(sender);
            thread::spawn(move || {
                loop {
                    match receiver.recv().unwrap() {
                        SearchMessage::Stop => todo!(),
                        SearchMessage::PonderHit => todo!(),
                    }
                }
            });
        }

        let mut pv_table = PVTable::new();
    
        //println!("Estimated CP score: {score}");
    
        print!("bestmove {}\n", pv_table.best_move().unwrap());
    }

    pub fn notify_stop(&self) {
        self.send(SearchMessage::Stop)
    }

    pub fn notify_ponder_hit(&self) {
        self.send(SearchMessage::PonderHit)
    }

    /// Send to search if it is running
    fn send(&self, msg: SearchMessage) {
        if self.is_running.load(Relaxed) {
            self.sender.lock().unwrap().unwrap().send(msg);
        } else {
            println!("Can't stop search, as there is no search running")
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Relaxed)
    }

    fn mark_as_done(&self) {
        self.is_running.store(false, Relaxed)
    }

    /// Resets the transposition table etc.
    pub fn new_game(&self) {

    }
}

/// The arguments provided in go command
#[derive(Clone)]
pub struct SearchInfo {
    pos: Position,
    depth: u8,
}

impl SearchInfo {
    pub fn new(pos: Position, depth: u8) -> Self {
        Self { pos, depth }
    }
}

/// Messages sent from main to search thread
enum SearchMessage {
    Stop,
    PonderHit,
}

#[allow(dead_code)]
pub struct SearchContext {
    search: Search,
    search_info: SearchInfo,
    pos: Position,
    receiver: Receiver<SearchMessage>,
}

impl SearchContext {
    pub fn new(search: Search, search_info: SearchInfo, pos: Position, receiver: Receiver<SearchMessage>) -> Self {
        Self {
            search,
            search_info,
            pos,
            receiver,
        }
    }
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