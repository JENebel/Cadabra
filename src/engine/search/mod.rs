mod evaluation;
mod perft;
mod search;
mod transposition_table;
mod pv_table;

//use transposition_table::*;
pub use evaluation::*;
pub use search::*;
pub use perft::*;
use pv_table::*;

use crate::engine::*;

pub const MAX_PLY: usize = 64;