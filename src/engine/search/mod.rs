mod evaluation;
mod perft;
mod search;
mod transposition_table;
mod tri_pv_table;
mod score_move;
mod search_context;
mod repetition_table;
mod search_args;
mod search_stats;

//use transposition_table::*;
pub use evaluation::*;
pub use search::*;
pub use perft::*;
pub use transposition_table::*;
pub use search_context::*;
pub use tri_pv_table::*;
pub use repetition_table::*;
pub use search_args::*;
pub use search_stats::*;

use crate::engine::*;

pub const MAX_PLY: u8 = 64;

/// Considered infinite
pub const INFINITY: i16 = 30000;

/// Value when in mate
pub const MATE_VALUE: i16 = 29000;
/// Assume everything between this and MATE_VALUE is mate
pub const MATE_BOUND: i16 = 28000;

/// Initial aspiration window is +- this value
pub const ASPIRATION_WINDOW: i16 = 15;
/// Exponentially increase window by this multiplier on fail
pub const ASPIRATION_WINDOW_MULT: i16 = 3;

/// Amount of killer moves pr. ply to remember. Must be at least 1
pub const KILLER_MOVE_COUNT: usize = 3;