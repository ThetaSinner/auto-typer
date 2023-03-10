use std::path::PathBuf;

pub struct Config {
    pub file: PathBuf,
    pub start_delay: u8,
    pub wpm: f64,
}
