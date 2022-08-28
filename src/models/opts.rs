use std::path::PathBuf;

use clap::Parser;


#[derive(Parser)]
pub struct Opts {
    /// Path to server configuration file
    pub config_path: PathBuf
}