use crate::version::Version;

#[derive(Debug)]
pub struct Config {
    pub compression_threshold: usize,
    pub version: Version,
}

impl Config {
    pub fn new(version: Version) -> Self {
        Self {
            compression_threshold: 256,
            version,
        }
    }
}
