pub const PKG_NAME: &str = "Cadabra";
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod core;
mod interface;
mod search;
mod bench;

pub use crate::core::*;
pub use interface::*;
pub use bench::*;
pub use search::*;

pub struct Settings {
    pub threads: u8,
    pub transposition_table_mb: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self { threads: 1, transposition_table_mb: 128 }
    }
}