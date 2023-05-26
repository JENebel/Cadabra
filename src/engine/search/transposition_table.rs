use std::{mem::{size_of, self}, sync::atomic::AtomicU64};
use std::sync::atomic::Ordering::*;

use super::*;

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
    pub fn new(hash: u64, best_move: Move, depth: u8, score: i16, flag: HashFlag) -> Self {
        let mut data = best_move.data as u64;
        data |= (depth as u64) << 16;
        data |= (unsafe {mem::transmute::<i16, u16>(score)} as u64) << 24;
        data |= (flag as u64) << 40;

        // 48 bytes used for data. Rest should be used for age and maybe something more.

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
    table: Box<[(AtomicU64, AtomicU64)]>
}

impl TranspositionTable {
    const BYTES_PR_MB: usize = 1024 * 1024;

    pub fn new(megabytes: usize) -> Self {
        let bytes = Self::BYTES_PR_MB * megabytes;
        let entry_count = bytes / size_of::<TTEntry>(); // Makes entry_count a bitmap: 0b00...0011...11
        
        assert!((entry_count & (entry_count - 1)) == 0, "Transposition entry count must be power of 2");
        
        Self {
            table: vec![false; entry_count].iter().map(|_| (AtomicU64::from(0), AtomicU64::from(0))).collect::<Vec<(AtomicU64, AtomicU64)>>().into_boxed_slice()
        }
    }

    pub fn entry_count(&self) -> usize {
        self.table.len()
    }

    pub fn probe(&self, hash: u64) -> Option<TTEntry> {
        let index = (hash % self.entry_count() as u64) as usize;
        let entry = TTEntry::from((
            self.table[index].0.load(Relaxed),
            self.table[index].1.load(Relaxed),
        ));

        if (entry.hash ^ entry.data) == hash {
            Some(entry)
        } else {
            None
        }
    }

    pub fn record(&self, hash: u64, best_move: Option<Move>, depth: u8, score: i16, flag: HashFlag, ply: u8) {
        // Adjust mating scores here
        let score = if score < -MATE_BOUND {
            score - ply as i16
        } else if score > MATE_BOUND {
            score + ply as i16
        } else {
            score
        };

        let entry = TTEntry::new(hash, best_move.unwrap_or(Move::NULL), depth, score, flag);
        
        let index = (hash % self.entry_count() as u64) as usize;
        self.table[index].0.store(entry.hash ^ entry.data, Relaxed);
        self.table[index].1.store(entry.data, Relaxed);
    }
}