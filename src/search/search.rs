use std::sync::mpsc::Receiver;

use super::*;

/// The arguments provided in go command
pub struct SearchInfo {
    depth: u8,
}

impl SearchInfo {
    pub fn new(depth: u8) -> Self {
        Self { depth }
    }
}

/// Messages sent between main and search thread
pub enum SearchMessage {
    Done(Move),
    Stop,
    PonderHit,
}

pub struct SearchContext {
    search_info: SearchInfo, 
    pos: Position, 
    settings: Settings, 
    receiver: Receiver<SearchMessage>,
    is_stopping: bool,
}

impl SearchContext {
    pub fn new(search_info: SearchInfo, pos: Position, settings: Settings, receiver: Receiver<SearchMessage>) -> Self {
        Self {
            search_info,
            pos,
            settings,
            receiver,
            is_stopping: false
        }
    }
}

pub fn search(context: SearchContext) {
    match context.receiver.try_recv() {
        Ok(_) => println!("Done!"),
        Err(_) => println!("Done ERR!"),
    }
}