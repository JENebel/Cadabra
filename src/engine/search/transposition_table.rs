use std::{mem::{size_of, self}};
use std::sync::atomic::Ordering::*;

use portable_atomic::AtomicU128;

use super::*;

#[derive(PartialEq)]
pub enum HashFlag {
    Exact,
    UpperBound,
    LowerBound,
}

pub struct TTEntry {
    data: u128
}

impl TTEntry {
    pub fn new(hash: u64, best_move: Move, depth: u8, score: i16, flag: HashFlag) -> Self {
        let mut data = best_move.data as u128;
        data |= (depth as u128) << 16;
        data |= (unsafe {mem::transmute::<i16, u16>(score)} as u128) << 24;
        data |= (flag as u128) << 40;
        data |= (hash as u128) << 64;

        // 48 bytes used for data. Rest should be used for age or maybe bigger score value

        Self { data }
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
        (self.data >> 64) as u64
    }
}

impl From<u128> for TTEntry {
    fn from(data: u128) -> Self {
        Self { data }
    }
}

pub struct TranspositionTable {
    table: Box<[AtomicU128]>
}

impl TranspositionTable {
    const BYTES_PR_MB: usize = 1024 * 1024;

    pub fn new(megabytes: usize) -> Self {
        assert!((megabytes & (megabytes - 1)) == 0, "Transposition size must be power of 2");

        let bytes = Self::BYTES_PR_MB * megabytes;
        let entry_count = bytes / size_of::<TTEntry>(); // Makes entry_count a bitmap: 0b00...0011...11
        
        assert!((entry_count & (entry_count - 1)) == 0, "Transposition entry count must be power of 2");
        
        Self {
            table: vec![false; entry_count].iter().map(|_| AtomicU128::from(0)).collect::<Vec<AtomicU128>>().into_boxed_slice()
        }
    }

    pub fn entry_count(&self) -> usize {
        self.table.len()
    }

    fn load(&self, hash: u64) -> Option<TTEntry> {
        let index = (hash & (self.entry_count() as u64 - 1)) as usize;
        let data = self.table[index].load(Relaxed);
        let entry = TTEntry::from(data);
        if entry.hash() != hash {
            None
        } else {
            Some(entry)
        }
    }

    fn store(&self, hash: u64, entry: TTEntry) {
        let index = (hash & (self.entry_count() as u64 - 1)) as usize;
        self.table[index].store(entry.data, Relaxed);
    }

    pub fn record(&self, hash: u64, best_move: Option<Move>, depth: u8, score: i16, flag: HashFlag) {
        // Adjust mating scores here

        let entry = TTEntry::new(hash, best_move.unwrap_or(Move::NULL), depth, score, flag);
        self.store(hash, entry);
    }

    /// Returns (score, best_move, hash_flag) 
    pub fn probe(&self, hash: u64) -> Option<TTEntry> {
        //Adjust mating scores before extraction
        //let adjusted_score = data.score();
        /*if adjusted_score < -MATE_BOUND {
            adjusted_score += ply as i32;
        } else if adjusted_score > MATE_BOUND {
            adjusted_score -= ply as i32;
        }*/

        self.load(hash)

    }
}