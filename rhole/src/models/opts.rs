use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, about, long_about)]
pub struct Opts {
    /// Enables debug mode
    #[clap(short = 'd', long = "debug", env = "DEBUG")]
    pub debug: bool,
    /// Path to server configuration file
    #[clap(short = 'c', long = "config", env = "CONFIG_PATH")]
    pub config_path: PathBuf,
    /// Do not update database from sources
    #[clap(short = 'n', long = "no-update-db")]
    pub no_update_db: bool,
}
