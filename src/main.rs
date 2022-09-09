mod controllers;
mod models;
pub use crate::models::Config;

use anyhow::Result;
use clap::Parser;
use controllers::{BlacklistController, InboundConnectionsController};
use models::Opts;

// TODO:
// - RUST WEB frontend ?

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    log::debug!("Starting...");

    let opts = Opts::parse();
    let config = Config::from_file(opts.config_path).await?;
    let blacklist_controller = BlacklistController::init_from_sources(config.sources).await;

    let inbound_connections_controller =
        InboundConnectionsController::new(config.net, config.proxy_server, blacklist_controller.get_blacklist());

    inbound_connections_controller.listen().await?;

    Ok(())
}
