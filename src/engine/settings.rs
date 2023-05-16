#[derive(Copy, Clone)]
pub struct Settings {
    pub threads: u8,
    pub transposition_table_mb: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            threads: 1,
            transposition_table_mb: 16
        }
    }
}