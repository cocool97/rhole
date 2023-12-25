#![forbid(unsafe_code)]

mod api_models;
mod controllers;
mod graphql;
mod handlers;
mod models;
mod rhole_server;
mod utils;

use anyhow::Result;
use clap::Parser;
use models::Opts;
use rhole_server::RholeServer;

use crate::models::ServerConfig;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    utils::set_log_level(opts.debug);

    log::info!(
        "Starting rhole server version {}...",
        env!("CARGO_PKG_VERSION")
    );

    let f = File::open(&opts.config_path).await?;
    let config: ServerConfig = serde_yaml::from_reader(f.into_std().await)?;

    RholeServer::run(opts, config).await
}
