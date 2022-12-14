use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, about, long_about)]
pub struct Opts {
    #[command(subcommand)]
    pub command: RholeCommand,
}

#[derive(Subcommand)]
pub enum RholeCommand {
    /// Runs rhole
    Start {
        /// Enables debug mode
        #[clap(short = 'd', long = "debug")]
        debug: bool,
        /// Path to server configuration file
        #[clap(short = 'c', long = "config")]
        config_path: PathBuf,
    },
    /// Displays build informations
    Info,
}
