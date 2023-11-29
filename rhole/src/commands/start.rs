use std::{
    io::BufReader,
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime},
};

use crate::{
    api_models::ServerConfig,
    handlers::{graphiql_playground, graphql},
    models::{GraphQLState, RouterState},
};
use anyhow::{anyhow, Result};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::routing::get;
use axum_server::tls_rustls::RustlsConfig;
use hickory_server::ServerFuture;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::{
    fs::File,
    net::{TcpListener, UdpSocket},
};
use tower_http::services::ServeDir;

use crate::{
    controllers::{BlacklistController, DatabaseController, RequestsController},
    graphql::RholeQueries,
    models::RouterData,
    utils,
};

pub async fn start(
    debug: bool,
    config_path: PathBuf,
    no_update_config: bool,
    http: bool,
) -> Result<()> {
    let start_time = SystemTime::now();

    utils::set_log_level(debug);

    log::info!(
        "Starting rhole server version {}...",
        env!("CARGO_PKG_VERSION")
    );

    let f = File::open(&config_path).await?;
    let config: ServerConfig = serde_yaml::from_reader(f.into_std().await)?;

    let database_controller = DatabaseController::init_database(&config.database_path).await?;

    let blacklist_controller = match no_update_config {
        true => BlacklistController::new(database_controller.clone()),
        false => {
            BlacklistController::init_from_sources(
                &config.sources.entries,
                database_controller.clone(),
            )
            .await?
        }
    };

    let dns_socket = UdpSocket::bind((
        config.net.dns.listen_addr.as_str(),
        config.net.dns.listen_port,
    ))
    .await?;

    let tcp_listener = TcpListener::bind((
        config.net.dot.listen_addr.as_str(),
        config.net.dot.listen_port,
    ))
    .await?;

    let cert_file = File::open(&config.tls.certificate_path).await?;
    let key_file = File::open(&config.tls.pkey_path).await?;

    let cert_file = &mut BufReader::new(cert_file.into_std().await);
    let key_file = &mut BufReader::new(key_file.into_std().await);

    let cert_chain: Vec<Certificate> = certs(cert_file)?.into_iter().map(Certificate).collect();
    let key = pkcs8_private_keys(key_file)?
        .first()
        .ok_or(anyhow!("No key found..."))?
        .to_owned();

    let mut server = ServerFuture::new(
        RequestsController::new(
            config.proxy_server.clone(),
            blacklist_controller,
            &config.local_hosts,
        )
        .await?,
    );
    server.register_socket(dns_socket);
    server.register_tls_listener(
        tcp_listener,
        Duration::from_secs(config.net.dot.timeout),
        (cert_chain.to_vec(), PrivateKey(key.clone())),
    )?;

    tokio::spawn(async move { server.block_until_done().await });

    let graphql_state = GraphQLState {
        start_time,
        config: config.clone(),
        database_controller,
    };

    let graphql_schema = Schema::build(RholeQueries::default(), EmptyMutation, EmptySubscription)
        .data(graphql_state)
        .finish();

    let router_state = RouterState {
        router_data: Arc::new(RouterData { graphql_schema }),
    };

    let router = axum::Router::new()
        .route("/graphql", get(graphiql_playground).post(graphql))
        .route_service("/", ServeDir::new(config.html_dir))
        .route("/echo", get(|| async { env!("CARGO_PKG_VERSION") }))
        .with_state(router_state);

    let listen_addr: SocketAddr = format!(
        "{}:{}",
        config.net.web_interface.listen_addr.as_str(),
        config.net.web_interface.listen_port.to_string().as_str(),
    )
    .parse()?;

    match http {
        true => {
            axum_server::bind(listen_addr)
                .serve(router.into_make_service())
                .await?;
        }
        false => {
            axum_server::bind_rustls(listen_addr, tls_config(&cert_chain, &key).await?)
                .serve(router.into_make_service())
                .await?;
        }
    }

    Ok(())
}

async fn tls_config(cert_chain: &[Certificate], key: &[u8]) -> Result<RustlsConfig> {
    let mut config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain.to_vec(), PrivateKey(key.to_vec()))?;

    config.alpn_protocols.push(b"http/1.1".to_vec());
    config.alpn_protocols.push(b"h2".to_vec());

    Ok(RustlsConfig::from_config(Arc::new(config)))
}
