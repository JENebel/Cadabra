mod evaluation;
mod perft;
mod search;
mod transposition_table;
mod tri_pv_table;
mod score_move;
mod search_structs;

//use transposition_table::*;
pub use evaluation::*;
pub use search::*;
pub use perft::*;
pub use transposition_table::*;
pub use search_structs::*;
pub use tri_pv_table::*;

use crate::engine::*;

pub const MAX_PLY: u8 = 64;
pub const INFINITY: i16 = 30000;
pub const MATE_VALUE: i16 = 29000;
pub const MATE_BOUND: i16 = 28000;