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

impl Settings {
    pub fn threads(mut self, threads: u8) -> Self {
        self.threads = threads;
        self
    }

    pub fn transposition_table_mb(mut self, transposition_table_mb: usize) -> Self {
        self.transposition_table_mb = transposition_table_mb;
        self
    }
}