use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The input file to load
    #[arg(short, long, value_name = "FILE")]
    pub(crate) file: PathBuf,

    /// Delay seconds after confirming the start of a stage
    #[arg(long, default_value_t = 5, value_name = "SECONDS")]
    pub(crate) start_delay: u8,

    /// Delay milliseconds between key presses
    #[arg(long, default_value_t = 250., value_name = "MS")]
    pub(crate) input_delay: f64,
}
