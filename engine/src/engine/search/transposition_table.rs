use std::{mem::{size_of, self}, sync::atomic::AtomicU64};
use std::sync::atomic::Ordering::*;

use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum HashFlag {
    Exact,
    UpperBound,
    LowerBound,
}

// Entry bit layout:
// 16 bits     Hash
// 16 bits     Score
// 8  bits     Depth
// 16 bits     Move
// 2  bits     Flag
// 6  bits     Generation

pub struct AtomicEntry {
    hash: AtomicU64,
    data: AtomicU64,
}

impl Default for AtomicEntry {
    fn default() -> Self {
        Self {
            hash: AtomicU64::new(0),
            data: AtomicU64::new(0),
        }
    }
}

impl AtomicEntry {
    /// Fails if hash does not match, and on race conditions
    fn load_valid(&self, hash: u64) -> Result<EntryData, ()> {
        let entry_hash = self.hash.load(Relaxed);
        let data = self.data.load(Relaxed);

        if hash != entry_hash ^ data {
            return Err(())
        }

        Ok(EntryData::new_from_data(entry_hash, data))
    }

    /// Does not handle race conditions!
    fn load_unsafe(&self) -> EntryData {
        let hash = self.hash.load(Relaxed);
        let data = self.data.load(Relaxed);

        EntryData::new_from_data(hash, data)
    }

    fn store(&self, data: EntryData) {
        let hash = data.hash;
        let data = data.compress();
        self.hash.store(hash ^ data , Relaxed);
        self.data.store(data, Relaxed);
    }
}

/// Transposition table entry
pub struct EntryData {
    pub hash: u64,
    pub score: i16,
    pub depth: u8,
    pub best_move: Move,
    pub flag: HashFlag,
    pub generation: u8,
}

impl EntryData {
    fn new_from_data(hash: u64, data: u64) -> Self {
        Self {
            hash,
            score: data as i16,
            depth: (data >> 32) as u8,
            best_move: Move::from((data >> 40) as u16),
            flag: unsafe { mem::transmute::<u8, HashFlag>(((data >> 56) & 0b11) as u8) },
            generation: (data >> 58) as u8,
        }
    }

    pub fn new(hash: u64, score: i16, depth: u8, best_move: Move, flag: HashFlag, generation: u8) -> Self {
        Self {
            hash,
            score,
            depth,
            best_move,
            flag,
            generation,
        }
    }

    pub fn compress(&self) -> u64 {
        ((self.score as u16) as u64)
        | (self.depth as u64) << 32
        | (self.best_move.data as u64) << 40
        | (self.flag as u64) << 56
        | (self.generation as u64) << 58
    }
}

pub struct TranspositionTable {
    table: Vec<AtomicEntry>,
}

impl TranspositionTable {
    const BYTES_PR_MB: usize = 1024 * 1024;

    pub fn new(megabytes: usize) -> Self {
        let bytes = Self::BYTES_PR_MB * megabytes;

        let entry_count = bytes / size_of::<AtomicEntry>();
        
        Self {
            table: vec![(); entry_count].iter().map(|_| AtomicEntry::default()).collect()
        }
    }

    fn index(&self, hash: u64) -> usize {
        hash as usize % self.table.len()
    }

    /// Probe the transposition table for a hash. Returns None if no entry is found.
    pub fn probe(&self, hash: u64, ply: u8) -> Option<EntryData> {
        let entry = self.table[self.index(hash)].load_valid(hash);

        match entry {
            Ok(mut entry) => {
                // Adjust mating scores here
                if entry.score < -MATE_BOUND {
                    entry.score += ply as i16
                } else if entry.score > MATE_BOUND {
                    entry.score -= ply as i16
                }

                Some(entry)
            },
            Err(_) => None,
        }
    }

    pub fn record(&self, hash: u64, best_move: Move, depth: u8, score: i16, flag: HashFlag, ply: u8, generation: u8) {
        // Adjust mating scores here before storing
        let score = if score < -MATE_BOUND {
            score - ply as i16
        } else if score > MATE_BOUND {
            score + ply as i16
        } else {
            score
        };

        let entry = EntryData::new(hash, score, depth, best_move, flag, generation);
        
        self.table[self.index(hash)].store(entry)
    }

    pub fn clear(&self) {
        for entry in self.table.iter() {
            entry.data.store(0, Relaxed);
            entry.hash.store(0, Relaxed);
        }
    }

    /// Probes first 1000 buckets to estimate fill rate (0 = 0%, 1 = 100%)
    pub fn fill_rate(&self) -> f32 {
        let mut checked = 0;
        let mut found = 0;
        for entry in self.table.iter().take(1000) {
            let entry = entry.load_unsafe();
            if entry.hash != 0 {
                found += 1;
            }
            checked += 1;
        }

        return found as f32 / checked as f32;
    }
}