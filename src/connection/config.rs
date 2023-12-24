pub struct Config {
    pub compression_threshold: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            compression_threshold: 256,
        }
    }
}
