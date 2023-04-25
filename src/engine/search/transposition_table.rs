use std::{mem::{size_of, self}, sync::atomic::AtomicU64};
use std::sync::atomic::Ordering::*;

use super::*;

#[derive(PartialEq)]
pub enum HashFlag {
    Empty = 0,
    Exact,
    Alpha,
    Beta,
}

#[derive(Copy, Clone, Default)]
pub struct EntryData {
    data: u64
}

impl EntryData {
    pub fn new(best_move: Move, depth: u8, score: i16, flag: HashFlag) -> Self {
        let mut data = best_move.data as u64;
        data |= (depth as u64) << 16;
        data |= (unsafe {mem::transmute::<i16, u16>(score)} as u64) << 24;
        data |= (flag as u64) << 40;

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

    pub fn hash_flag(&self) -> HashFlag {
        unsafe { mem::transmute::<u8, HashFlag>((self.data >> 40) as u8) }
    }
}

#[repr(C, align(128))]
struct TTEntry {
    hash: AtomicU64,
    data: AtomicU64
}

impl TTEntry {
    pub fn empty() -> Self {
        Self {
            hash: AtomicU64::from(0),
            data: AtomicU64::from(0)
        }
    }
}

pub struct TranspositionTable {
    table: Box<[TTEntry]>
}

impl TranspositionTable {
    const BYTES_PR_MB: usize = 1024 * 1024;

    pub fn new(megabytes: usize) -> Self {
        assert!((megabytes & (megabytes - 1)) == 0, "Transposition size must be power of 2"); // Must be power of 2

        let bytes = Self::BYTES_PR_MB * megabytes;
        let entry_count = bytes / size_of::<TTEntry>(); // Makes entry_count a bitmap: 0b00...0011...11
        
        Self {
            table: vec![false; entry_count].iter().map(|_| TTEntry::empty()).collect::<Vec<TTEntry>>().into_boxed_slice()
        }
    }

    pub fn entry_count(&self) -> usize {
        self.table.len()
    }

    #[inline(never)]
    pub fn record(&self, hash: u64, best_move: Option<Move>, depth: u8, score: i16, flag: HashFlag) {
        // Adjust mating scores here

        let index = (hash & (self.entry_count() as u64 - 1)) as usize;
        let data = EntryData::new(best_move.unwrap_or(Move::NULL), depth, score, flag).data;
        self.table[index].hash.store(hash ^ data, Relaxed);
        self.table[index].data.store(data, Relaxed);
    }

    #[inline(never)]
    fn read_hash(&self, index: usize) -> u64 {
        self.table[index].hash.load(Acquire)
    }

    #[inline(never)]
    fn read_data(&self, index: usize) -> EntryData {
        EntryData { data: self.table[index].data.load(Acquire) }
    }

    #[inline(never)]
    fn read(&self, hash: u64, depth: u8) -> EntryData {
        let index = (hash & (self.entry_count() as u64 - 1)) as usize;
        let ehash = self.read_hash(index);//self.table[index].hash.load(Relaxed);
        let edata = self.read_data(index); //EntryData { data: self.table[index].data.load(Relaxed) };

        if ehash ^ edata.data != hash || edata.depth() < depth {
            return EntryData::default();
        }

        return edata
    }

    #[inline(never)]
    /// Returns (score, best_move, hash_flag) 
    pub fn probe(&self, hash: u64, depth: u8) -> (i16, Move, HashFlag) {
        let data = self.read(hash, depth);

        //Adjust mating scores before extraction
        let adjusted_score = data.score();
        /*if adjusted_score < -MATE_BOUND {
            adjusted_score += ply as i32;
        } else if adjusted_score > MATE_BOUND {
            adjusted_score -= ply as i32;
        }*/

        let moove = data.best_move();
        let flag = data.hash_flag();

        (adjusted_score, moove, flag)
    }
}