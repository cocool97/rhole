use std::path::PathBuf;

use anyhow::Result;
use tokio::net::UdpSocket;
use trust_dns_server::ServerFuture;

use crate::{
    controllers::{BlacklistController, DatabaseController, RequestsController},
    models::Config,
    utils,
};

pub async fn start(debug: bool, config_path: PathBuf) -> Result<()> {
    utils::set_log_level(debug);

    log::info!("Starting...");

    let config = Config::from_file(config_path).await?;

    let database_controller = DatabaseController::init_database(&config.database.stats).await?;

    let blacklist_controller =
        BlacklistController::init_from_sources(config.sources.entries, config.database).await?;

    let socket = UdpSocket::bind((config.net.listen_addr.as_str(), config.net.listen_port)).await?;

    let mut server = ServerFuture::new(
        RequestsController::new(
            blacklist_controller.get_blacklist(),
            config.proxy_server,
            database_controller,
        )
        .await?,
    );
    server.register_socket(socket);

    Ok(server.block_until_done().await?)
}
