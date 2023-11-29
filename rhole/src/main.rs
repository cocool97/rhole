#![forbid(unsafe_code)]

mod api_models;
mod commands;
mod controllers;
mod graphql;
mod handlers;
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
        RholeCommand::Start {
            debug,
            config_path,
            no_update_db,
            http,
        } => commands::start(debug, config_path, no_update_db, http).await?,
        RholeCommand::Info => commands::info(),
    }

    Ok(())
}
