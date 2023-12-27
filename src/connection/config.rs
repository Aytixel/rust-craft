use std::time::Duration;

use crate::version::Version;

#[derive(Debug)]
pub struct Config {
    pub compression_threshold: usize,
    pub timeout: Duration,
    pub version: Version,
}

impl Config {
    pub fn new(version: Version) -> Self {
        Self {
            compression_threshold: 256,
            timeout: Duration::from_secs(10),
            version,
        }
    }
}
