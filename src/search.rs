/*use std::sync::{Mutex, Arc, atomic::{AtomicU8, AtomicI32, AtomicBool}, mpsc::{channel, Sender, Receiver}};

use crate::{Position/*, MoveList*/};

enum ThreadMsg {

}

struct Worker {
    is_running: AtomicBool,
    last_split_node: Node,
}

struct WorkerPool {
    workers: Vec<Worker>,
}

struct Node {
    move_list: Arc<Mutex<MoveList>>,
    ply: AtomicU8,
    alpha: AtomicI32,
    threads_here: AtomicU8,
}

pub fn search(pos: &Position, depth: u8) {
    let score = negamax(pos, -50000, 50000, depth, 0);

    println!("Estimated CP score: {score}");

    let (rx, tx) = channel::<ThreadMsg>();

    spawn_worker(tx);
}

fn spawn_worker(tx: Receiver<ThreadMsg>) -> Sender<ThreadMsg> {
    let (rx, _) = channel::<ThreadMsg>();
    
    rx
}

fn negamax(pos: &Position, mut alpha: i32, beta: i32, depth: u8, thread_id: u16) -> i32 {
    if depth == 0 {
        return pos.evaluate()
    };

    let move_list = pos.generate_moves();

    for m in move_list {
        let mut new_pos = pos.clone();

        new_pos.make_move(m);

        let score = -negamax(&new_pos, -beta, -alpha, depth - 1, thread_id);

        if score > alpha {
            if score >= beta {
                return beta;
            }

            alpha = score;
        }
    }

    alpha
}*/