use std::mem::{size_of, self};

use super::*;

#[derive(PartialEq)]
pub enum NodeType {
    Exact,
    Alpha,
    Beta,
}

#[derive(Copy, Clone, Default)]
pub struct EntryData {
    data: u64
}

impl EntryData {
    pub fn new(best_move: Move, depth: u8, score: i16, node_type: NodeType) -> Self {
        let mut data = best_move.data as u64;
        data |= (depth as u64) << 16;
        data |= (unsafe {mem::transmute::<i16, u16>(score)} as u64) << 24;
        data |= (node_type as u64) << 40;

        // 48 bytes used. Rest should be used for age or maybe bigger score value

        Self { data }
    }

    pub fn is_some(&self) -> bool {
        self.data != 0
    }

    pub fn best_move(&self) -> Move {
        Move {
            data: self.data as u16
        }
    }

    pub fn depth(&self) -> u8 {
        (self.data >> 16) as u8
    }

    pub fn score(&self) -> i16 {
        unsafe { mem::transmute::<u16, i16>((self.data >> 24) as u16) }
    }

    fn node_type(&self) -> NodeType {
        unsafe { mem::transmute::<u8, NodeType>((self.data >> 40) as u8) }
    }
}

#[derive(Copy, Clone, Default)]
struct TTEntry {
    hash: u64,
    data: EntryData
}

impl TTEntry {
    pub fn new(hash: u64, data: EntryData) -> Self {
        Self {
            hash,
            data
        }
    }

    pub fn is_some(&self) -> bool {
        self.data.is_some()
    }

    pub const EMPTY: Self = Self {
        hash: 0,
        data: EntryData { data: 0 }
    };
}

#[derive(Clone)]
pub struct TranspositionTable {
    table: Box<[TTEntry]>
}

impl TranspositionTable {
    const BYTES_PR_MB: usize = 1024 * 1024;

    #[inline(never)]
    pub fn new(megabytes: usize) -> Self {
        let bytes = Self::BYTES_PR_MB * megabytes;
        let entry_count = bytes / size_of::<TTEntry>();
        
        Self {
            table: vec![TTEntry::EMPTY; entry_count].into_boxed_slice()
        }
    }

    pub fn entry_count(&self) -> usize {
        self.table.len()
    }

    pub fn record(&mut self, hash: u64, best_move: Move, depth: u8, score: i16, node_type: NodeType) {
        // Adjust mating scores here

        let index = (hash % self.entry_count() as u64) as usize;
        self.table[index] = TTEntry::new(hash, EntryData::new(best_move, depth, score, node_type));
    }

    pub fn record_alpha(&mut self, hash: u64, depth: u8, beta: i16) {
        // Adjust mating scores here

        let index = (hash % self.entry_count() as u64) as usize;
        self.table[index] = TTEntry::new(hash, EntryData::new(unsafe { mem::transmute::<u16, Move>(0)}, depth, beta, NodeType::Beta));
    }

    pub fn probe(&self, hash: u64, depth: u8, alpha: i16, beta: i16) -> Option<i16> {
        let index = (hash % self.entry_count() as u64) as usize;
        let entry = &self.table[index];

        if entry.is_some() && entry.hash == hash {
            if entry.data.depth() >= depth {
                //Adjust mating scores before extraction
                let adjusted_score = entry.data.score();
                /*if adjusted_score < -MATE_BOUND {
                    adjusted_score += ply as i32;
                } else if adjusted_score > MATE_BOUND {
                    adjusted_score -= ply as i32;
                }*/


                if entry.data.node_type() == NodeType::Exact {
                    return Some(adjusted_score)
                }
                else if entry.data.node_type() == NodeType::Alpha && adjusted_score <= alpha {
                    return Some(alpha)
                }
                else if entry.data.node_type() == NodeType::Beta && adjusted_score >= beta {
                    return Some(beta)
                }
            }
        }

        return None;
    }
}