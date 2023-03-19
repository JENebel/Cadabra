mod attack_tables;
mod castling;
mod generate_moves;
mod move_list;

use attack_tables::*;
use move_list::*;
pub use castling::*;
pub use generate_moves::*;

pub use super::*;

pub fn init_attack_tables() {
    lazy_static::initialize(&SLIDING_ATTACKS);
}