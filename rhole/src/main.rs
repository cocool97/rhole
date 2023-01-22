#![forbid(unsafe_code)]

mod api;
mod commands;
mod controllers;
mod models;
mod utils;

use crate::models::RholeCommand;

use anyhow::Result;
use clap::Parser;
use models::Opts;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.command {
        RholeCommand::Start { debug, config_path } => commands::start(debug, config_path).await?,
        RholeCommand::Info => commands::info(),
    }

    Ok(())
}
