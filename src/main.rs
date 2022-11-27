#![forbid(unsafe_code)]

mod controllers;
mod models;
mod utils;

use crate::controllers::RequestsController;
pub use crate::models::Config;

use anyhow::Result;
use clap::Parser;
use controllers::BlacklistController;
use models::Opts;
use tokio::net::UdpSocket;
use trust_dns_server::ServerFuture;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    utils::set_log_level(opts.debug);

    log::info!("Starting...");

    let config = Config::from_file(opts.config_path).await?;

    let blacklist_controller =
        BlacklistController::init_from_sources(config.sources.entries, config.database).await?;

    let socket = UdpSocket::bind((config.net.listen_addr.as_str(), config.net.listen_port)).await?;

    let mut server = ServerFuture::new(
        RequestsController::new(
            blacklist_controller.get_blacklist(),
            config.proxy_server.clone(),
        )
        .await?,
    );
    server.register_socket(socket);
    server.block_until_done().await?;

    Ok(())
}
