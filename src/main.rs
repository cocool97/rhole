mod controllers;
mod models;
mod utils;

pub use crate::models::Config;

use anyhow::Result;
use clap::Parser;
use controllers::{BlacklistController, InboundConnectionsController};
use models::Opts;

// TODO:
// - RUST WEB frontend ?

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    utils::set_log_level(opts.debug);

    log::info!("Starting...");

    let config = Config::from_file(opts.config_path).await?;
    let blacklist_controller = BlacklistController::init_from_sources(config.sources).await;

    let inbound_connections_controller = InboundConnectionsController::new(
        config.proxy_server,
        blacklist_controller.get_blacklist(),
    );

    inbound_connections_controller
        .listen(config.net.listen_addr.as_str(), config.net.listen_port)
        .await?;

    Ok(())
}
