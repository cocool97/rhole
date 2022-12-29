use std::path::PathBuf;

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use anyhow::Result;
use tokio::net::UdpSocket;
use trust_dns_server::ServerFuture;

use crate::{
    controllers::{BlacklistController, DatabaseController, RequestsController},
    models::{AppData, Config},
    utils,
    web_handlers::blocked_requests,
};

pub async fn start(debug: bool, config_path: PathBuf) -> Result<()> {
    utils::set_log_level(debug);

    log::info!("Starting...");

    let config = Config::from_file(config_path).await?;

    let database_controller = DatabaseController::init_database(&config.database.stats).await?;

    let blacklist_controller =
        BlacklistController::init_from_sources(config.sources.entries, config.database).await?;

    let dns_socket = UdpSocket::bind((
        config.net.dns.listen_addr.as_str(),
        config.net.dns.listen_port,
    ))
    .await?;

    let mut server = ServerFuture::new(
        RequestsController::new(
            blacklist_controller.get_blacklist(),
            config.proxy_server,
            database_controller.clone(),
        )
        .await?,
    );
    server.register_socket(dns_socket);

    tokio::spawn(async { server.block_until_done().await });

    let app_data = Data::new(AppData {
        database_controller,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&app_data))
            .route("/blocked", web::get().to(blocked_requests))
            .default_service(
                actix_web::web::route()
                    .to(|| async { HttpResponse::MethodNotAllowed().body("Route not found...") }),
            )
    })
    .bind((
        config.net.web_interface.listen_addr.as_str(),
        config.net.web_interface.listen_port,
    ))?
    .run()
    .await?;

    Ok(())
}
