pub const PKG_NAME: &str = "Cadabra";
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod engine;
mod interface;
mod bench;

pub use crate::engine::*;
pub use interface::*;
pub use bench::*;