include!(concat!(env!("OUT_DIR"), "/consts.rs"));

pub const PKG_NAME: &str = "Cadabra";
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod utils;
mod bitboard;
mod position;
mod interface;
mod evaluation;
mod move_generator;
mod constants;
mod moov;
mod make_move;
mod perft;
mod bench;
mod search;

pub use make_move::*;
pub use bitboard::*;
pub use position::*;
pub use interface::*;
pub use evaluation::*;
pub use move_generator::*;
pub use utils::*;
pub use constants::*;
pub use moov::*;
pub use perft::*;
pub use bench::*;
pub use search::*;