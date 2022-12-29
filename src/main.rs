#![forbid(unsafe_code)]

mod commands;
mod controllers;
mod models;
mod utils;
mod web_handlers;

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
