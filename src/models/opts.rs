use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Opts {
    /// Enables debug mode
    #[clap(short = 'd', long = "debug")]
    pub debug: bool,
    /// Path to server configuration file
    #[clap(short = 'c', long = "config")]
    pub config_path: PathBuf,
}
