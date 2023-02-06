use std::{path::PathBuf, time::SystemTime};

use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use common::ServerConfig;
use tokio::{fs::File, net::UdpSocket};
use trust_dns_server::ServerFuture;

use crate::{
    api::handlers::{api_route_not_found, blocked_requests, clients, infos},
    controllers::{BlacklistController, DatabaseController, RequestsController},
    models::AppData,
    utils,
};

pub async fn start(debug: bool, config_path: PathBuf) -> Result<()> {
    let start_time = SystemTime::now();

    utils::set_log_level(debug);

    log::info!("Starting rhole server...");

    let f = File::open(&config_path).await?;
    let config: ServerConfig = serde_yaml::from_reader(f.into_std().await)?;

    let database_controller = DatabaseController::init_database(&config.database.stats).await?;

    let blacklist_controller =
        BlacklistController::init_from_sources(&config.sources.entries, &config.database).await?;

    let dns_socket = UdpSocket::bind((
        config.net.dns.listen_addr.as_str(),
        config.net.dns.listen_port,
    ))
    .await?;

    let mut server = ServerFuture::new(
        RequestsController::new(
            blacklist_controller.get_blacklist(),
            config.proxy_server.clone(),
            database_controller.clone(),
        )
        .await?,
    );
    server.register_socket(dns_socket);

    tokio::spawn(async { server.block_until_done().await });

    let app_data = Data::new(AppData {
        config: config.clone(),
        database_controller,
        start_time,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&app_data))
            .service(
                web::scope("/api")
                    .route("/blocked", web::get().to(blocked_requests))
                    .route("/clients", web::get().to(clients))
                    .route("/infos", web::get().to(infos))
                    .route("/config", web::get().to(crate::api::handlers::config))
                    .default_service(web::route().to(api_route_not_found)),
            )
            .default_service({
                let index_file = config.web_resources.index_file.clone();
                let static_files = config.web_resources.static_files.clone();

                Files::new(
                    &config.web_resources.mount_path,
                    &config.web_resources.static_files,
                )
                .index_file(&config.web_resources.index_file)
                .default_handler({
                    fn_service(move |req: ServiceRequest| {
                        let index_file = index_file.clone();
                        let static_files = static_files.clone();
                        async move {
                            let (req, _) = req.into_parts();
                            let file = NamedFile::open_async(static_files.join(index_file)).await?;
                            let res = file.into_response(&req);
                            Ok(ServiceResponse::new(req, res))
                        }
                    })
                })
            })
    })
    .bind((
        config.net.web_interface.listen_addr.as_str(),
        config.net.web_interface.listen_port,
    ))?
    .run()
    .await?;

    Ok(())
}
