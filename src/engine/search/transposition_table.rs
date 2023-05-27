use std::{mem::{size_of, self}, sync::atomic::AtomicU64};
use std::sync::atomic::Ordering::*;

use super::*;

const BUCKET_SIZE: usize = 4;

#[derive(PartialEq)]
pub enum HashFlag {
    Exact,
    UpperBound,
    LowerBound,
}

pub struct TTEntry {
    hash: u64,
    data: u64
}

impl TTEntry {
    pub fn new(hash: u64, best_move: Move, depth: u8, score: i16, flag: HashFlag, age: u8) -> Self {
        let mut data = best_move.data as u64;
        data |= (depth as u64) << 16;
        data |= (unsafe {mem::transmute::<i16, u16>(score)} as u64) << 24;
        data |= (flag as u64) << 40;
        data |= (age as u64) << 48;

        // 56 bytes used for data.

        Self { hash, data }
    }

    pub fn best_move(&self) -> Move {
        let data = self.data as u16;
        Move { data }
    }

    pub fn depth(&self) -> u8 {
        (self.data >> 16) as u8
    }

    pub fn score(&self) -> i16 {
        unsafe { mem::transmute::<u16, i16>((self.data >> 24) as u16) }
    }

    pub fn hash_flag(&self) -> HashFlag {
        unsafe { mem::transmute::<u8, HashFlag>((self.data >> 40) as u8) }
    }

    pub fn age(&self) -> u8 {
        (self.data >> 48) as u8
    }

    pub fn hash(&self) -> u64 {
        self.hash 
    }
}

impl From<(u64, u64)> for TTEntry {
    fn from((hash, data): (u64, u64)) -> Self {
        Self { hash, data }
    }
}

pub struct TranspositionTable {
    table: Box<[(AtomicU64, AtomicU64)]>,
    entry_count: usize,
}

impl TranspositionTable {
    const BYTES_PR_MB: usize = 1024 * 1024;

    pub fn new(megabytes: usize) -> Self {
        let bytes = Self::BYTES_PR_MB * megabytes;

        let entry_count = bytes / size_of::<TTEntry>();
        
        assert!((entry_count & (entry_count - 1)) == 0, "Transposition entry count must be power of 2");
        
        Self {
            // entry_count + BUCKET_SIZE to avoid dropping out of bounds
            table: vec![false; entry_count + BUCKET_SIZE].iter().map(|_| (AtomicU64::from(0), AtomicU64::from(0))).collect::<Vec<(AtomicU64, AtomicU64)>>().into_boxed_slice(),
            entry_count
        }
    }

    fn index(&self, hash: u64) -> usize {
        (hash % self.entry_count as u64) as usize
    }

    /// Probe the transposition table for a hash. Returns None if no entry is found.
    pub fn probe(&self, hash: u64) -> Option<TTEntry> {
        let index = self.index(hash);

        // Linear probe
        for i in 0..BUCKET_SIZE {
            let entry = TTEntry::from((
                self.table[index + i].0.load(Relaxed),
                self.table[index + i].1.load(Relaxed),
            ));
    
            if (entry.hash ^ entry.data) == hash {
                return Some(entry)
            }
        }
        
        None
    }

    pub fn record(&self, hash: u64, best_move: Option<Move>, depth: u8, score: i16, flag: HashFlag, ply: u8, age: u8) {
        // Adjust mating scores here
        let score = if score < -MATE_BOUND {
            score - ply as i16
        } else if score > MATE_BOUND {
            score + ply as i16
        } else {
            score
        };

        // Linear probe
        let first_index = self.index(hash);
        let mut lowest_depth = MAX_PLY;
        let mut best_index = first_index;
        for i in 0..BUCKET_SIZE {
            let index = first_index + i;

            let entry = TTEntry::from((
                self.table[index].0.load(Relaxed),
                self.table[index].1.load(Relaxed),
            ));

            if entry.data == 0 || (entry.hash ^ entry.data) == hash {
                best_index = index;
                break;
            }

            let adjusted_depth = (entry.age() as i16 - 4 * age as i16).max(0) as u8;
            if adjusted_depth < lowest_depth {
                best_index = index;
                lowest_depth = adjusted_depth;
            }
        }

        let entry = TTEntry::new(hash, best_move.unwrap_or(Move::NULL), depth, score, flag, age);

        self.table[best_index].0.store(entry.hash ^ entry.data, Relaxed);
        self.table[best_index].1.store(entry.data, Relaxed);
    }
}