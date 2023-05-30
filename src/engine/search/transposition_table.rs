use std::{mem::{size_of, self}, sync::atomic::AtomicU64};
use std::sync::atomic::Ordering::*;
use cache_line_size::*;

use super::*;

const BUCKET_SIZE: usize = CACHE_LINE_SIZE / mem::size_of::<AtomicU64>();

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum HashFlag {
    Unused = 0,
    Exact = 1,
    UpperBound = 2,
    LowerBound = 3,
}

#[repr(align(64))]
pub struct Bucket {
    entries: [AtomicEntry; BUCKET_SIZE]
}

impl Default for Bucket {
    fn default() -> Self {
        Self { entries: Default::default() }
    }
}

// Entry bit layout:
// 16 bits     Hash
// 16 bits     Score
// 8  bits     Depth
// 16 bits     Move
// 2  bits     Flag
// 6  bits     Generation

pub struct AtomicEntry(AtomicU64);

impl Default for AtomicEntry {
    fn default() -> Self {
        Self(AtomicU64::new(0))
    }
}

impl AtomicEntry {
    fn load(&self) -> EntryData {
        EntryData::from(self.0.load(Relaxed))
    }

    fn store(&self, data: EntryData) {
        self.0.store(data.compress(), Relaxed);
    }
}

/// Transposition table entry
pub struct EntryData {
    key: u16,
    pub score: i16,
    pub depth: u8,
    pub best_move: Move,
    pub flag: HashFlag,
    generation: u8,
}

impl From<u64> for EntryData {
    fn from(data: u64) -> Self {
        Self {
            key: data as u16,
            score: (data >> 16) as i16,
            depth: (data >> 32) as u8,
            best_move: Move::from((data >> 40) as u16),
            flag: unsafe { mem::transmute::<u8, HashFlag>(((data >> 56) & 0b11) as u8) },
            generation: (data >> 58) as u8,
        }
    }
}

impl EntryData {
    pub fn new(hash: u64, score: i16, depth: u8, best_move: Move, flag: HashFlag, generation: u8) -> Self {
        Self {
            key: hash as u16,
            score,
            depth,
            best_move,
            flag,
            generation,
        }
    }

    pub fn compress(&self) -> u64 {
        self.key as u64
        | ((self.score as u16) as u64) << 16
        | (self.depth as u64) << 32
        | (self.best_move.data as u64) << 40
        | (self.flag as u64) << 56
        | (self.generation as u64) << 58
    }
}

pub struct TranspositionTable {
    table: Vec<Bucket>,
}

impl TranspositionTable {
    const BYTES_PR_MB: usize = 1024 * 1024;

    pub fn new(megabytes: usize) -> Self {
        let bytes = Self::BYTES_PR_MB * megabytes;

        let bucket_count = bytes / size_of::<Bucket>();
        
        Self {
            table: vec![(); bucket_count + BUCKET_SIZE].iter().map(|_| Bucket::default()).collect()
        }
    }

    fn get_bucket(&self, hash: u64) -> &Bucket {
        &self.table[hash as usize % self.table.len()]
    }

    /// Probe the transposition table for a hash. Returns None if no entry is found.
    pub fn probe(&self, hash: u64, ply: u8) -> Option<EntryData> {
        let bucket = self.get_bucket(hash);

        // Linear probe
        for entry in &bucket.entries {
            let mut entry = entry.load();

            if entry.flag != HashFlag::Unused && entry.key == hash as u16 {
                // Adjust mating scores here
                if entry.score < -MATE_BOUND {
                    entry.score += ply as i16
                } else if entry.score > MATE_BOUND {
                    entry.score -= ply as i16
                }

                return Some(entry)
            }
        }
        
        None
    }

    pub fn record(&self, hash: u64, best_move: Move, depth: u8, score: i16, flag: HashFlag, ply: u8, generation: u8) {
        let bucket = self.get_bucket(hash);

        let mut worst_index = 0;
        let mut worst_depth = i16::MAX;
        let mut old_move = Move::NULL;
        for i in 0..BUCKET_SIZE {
            let entry = bucket.entries[i].load();
    
            if entry.flag == HashFlag::Unused || entry.key == hash as u16 {
                worst_index = i;
                old_move = entry.best_move;
                break;
            }
            
            let age = generation as i16 - entry.generation as i16;
            let adjusted = entry.depth as i16 - age * AGE_REPLACEMENT_PENALTY;
            if adjusted < worst_depth {
                worst_index = i;
                worst_depth = adjusted;
                old_move = entry.best_move;
            }
        }

        // Adjust mating scores here before storing
        let score = if score < -MATE_BOUND {
            score - ply as i16
        } else if score > MATE_BOUND {
            score + ply as i16
        } else {
            score
        };

        let mut entry = EntryData::new(hash, score, depth, best_move, flag, generation);
        if best_move == Move::NULL {
            entry.best_move = old_move;
        }
        bucket.entries[worst_index].store(entry)
    }

    pub fn clear(&self) {
        for bucket in &self.table {
            for entry in &bucket.entries {
                entry.0.store(0, Relaxed);
            }
        }
    }

    /// Probes first 1000 buckets to estimate fill rate (0 = 0%, 1 = 100%)
    pub fn fill_rate(&self) -> f32 {
        let mut checked = 0;
        let mut found = 0;
        for bucket in &mut self.table.iter().take(1000) {
            for entry in &bucket.entries {
                let entry = entry.load();
                if entry.flag != HashFlag::Unused {
                    found += 1;
                }
                checked += 1;
            }
        }

        return found as f32 / checked as f32;
    }

    /// Probes first 1000 buckets to estimate fill rate per bucket slot (0 = 0%, 1 = 100%)
    pub fn fill_rate_detailed(&self) -> [f32; BUCKET_SIZE] {
        let mut checked = 0;
        let mut found = [0; BUCKET_SIZE];
        for bucket in &mut self.table.iter().take(1000) {
            for (i, entry) in bucket.entries.iter().enumerate() {
                let entry = entry.load();
                if entry.flag != HashFlag::Unused {
                    found[i] += 1;
                }
                checked += 1;
            }
        }

        return found.iter().map(|&x| x as f32 / checked as f32).collect::<Vec<f32>>().try_into().unwrap();
    }
}