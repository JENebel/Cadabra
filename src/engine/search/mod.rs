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