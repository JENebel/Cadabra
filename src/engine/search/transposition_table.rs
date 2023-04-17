use std::mem::{size_of, self};

use move_gen::zobrist::Zobrist;

use super::*;

#[derive(PartialEq)]
pub enum NodeType {
    Exact,
    UpperBound,
    LowerBound,
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
    hash: Zobrist,
    data: EntryData
}

impl TTEntry {
    pub fn new(hash: Zobrist, data: EntryData) -> Self {
        Self {
            hash,
            data
        }
    }
}

#[derive(Clone)]
pub struct TranspositionTable {
    table: Vec<TTEntry>
}

impl TranspositionTable {
    const BYTES_PR_MB: usize = 1048576;

    pub fn new(megabytes: usize) -> Self {
        let bytes = Self::BYTES_PR_MB * megabytes;
        let entry_count = bytes / size_of::<TTEntry>();
        
        Self {
            table: vec![Default::default(); entry_count]
        }
    }

    pub fn entry_count(&self) -> usize {
        self.table.len()
    }

    pub fn record(&mut self, hash: Zobrist, best_move: Move, depth: u8, score: i16, node_type: NodeType) {
        // Adjust mating scores here

        let index = (hash.hash % self.entry_count() as u64) as usize;
        self.table[index] = TTEntry::new(hash, EntryData::new(best_move, depth, score, node_type));
    }

    pub fn probe(&self, hash: Zobrist, depth: u8, alpha: i16, beta: i16) -> Option<i16> {
        let index = (hash.hash % self.entry_count() as u64) as usize;
        let entry = &self.table[index];

        if entry.hash == hash {
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
                else if entry.data.node_type() == NodeType::UpperBound && adjusted_score <= alpha {
                    return Some(alpha)
                }
                else if entry.data.node_type() == NodeType::LowerBound && adjusted_score >= beta {
                    return Some(beta)
                }
            }
        }

        return None;
    }
}