use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    /// Enables debug mode
    #[clap(short = 'd', long = "debug")]
    pub debug: bool,
    /// Path to server configuration file
    #[clap(short = 'c', long = "config")]
    pub config_path: PathBuf,
}
