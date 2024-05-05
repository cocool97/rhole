use std::{io::BufReader, process::exit, sync::Arc, time::SystemTime};

use anyhow::{anyhow, Result};
use async_graphql::Schema;
use async_graphql_axum::GraphQLSubscription;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use hickory_server::ServerFuture;
use log::{error, info};
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use rustls_pki_types::PrivatePkcs8KeyDer;
use tokio::{fs::File, net::UdpSocket};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    controllers::{BlacklistController, DatabaseController, RequestsController, WatcherController},
    graphql::{RholeMutations, RholeQueries, RholeSubscriptions},
    handlers::{graphiql_playground, graphql},
    models::{GraphQLState, Opts, RouterState, ServerConfig},
};

pub struct RholeServer {}

impl RholeServer {
    pub async fn run(opts: Opts, config: ServerConfig) -> Result<()> {
        let start_time = SystemTime::now();
        info!("DNS server listening on {}", opts.dns_addr);
        info!("Web server listening on {}", opts.web_addr);
        let database_controller = DatabaseController::init_database(&opts.database_path).await?;

        let blocked_requests_controller = WatcherController::new();
        let live_requests_controller = WatcherController::new();

        let blacklist_controller = BlacklistController::new(
            database_controller.clone(),
            blocked_requests_controller.clone(),
        );

        if !opts.no_update_db {
            // no_update_db is not set, we can update database entries
            // Spawns a task responsible of adding blacklist entries to database
            tokio::spawn({
                let bl = blacklist_controller.clone();
                let entries = config.sources.entries.to_owned();
                async move { bl.init_from_sources(&entries).await }
            });
        }

        let dns_socket = UdpSocket::bind(opts.dns_addr).await?;

        let mut server = ServerFuture::new(
            RequestsController::new(
                config.proxy_server.clone(),
                opts.cache_size,
                blacklist_controller,
                &config.local_hosts,
                live_requests_controller.clone(),
                database_controller.clone(),
            )
            .await?,
        );
        server.register_socket(dns_socket);

        tokio::spawn(async move {
            info!("DNS server up and running...");
            if let Err(e) = server.block_until_done().await {
                error!("{e}");
                exit(-1);
            }
        });

        let graphql_state = GraphQLState {
            start_time,
            config: config.clone(),
            database_controller,
            blocked_requests_controller,
            live_requests_controller,
        };

        let graphql_schema = Schema::build(
            RholeQueries::default(),
            RholeMutations::default(),
            RholeSubscriptions::default(),
        )
        .data(graphql_state)
        .finish();

        let router_state = RouterState {
            graphql_schema: graphql_schema.clone(),
            html_dir: opts.html_directory,
        };

        let api_router = Router::new()
            .route("/graphql", get(graphiql_playground).post(graphql))
            .route_service("/ws", GraphQLSubscription::new(graphql_schema));

        let router = Router::new()
            .route("/echo", get(|| async { env!("CARGO_PKG_VERSION") }))
            .nest("/api", api_router)
            .route("/index.html", get(|| async { Redirect::permanent("/") }))
            .fallback(handle_webapp)
            .with_state(router_state);

        match config.tls {
            None => {
                axum_server::bind(opts.web_addr)
                    .serve(router.into_make_service())
                    .await?;
            }
            Some(tls) => {
                let cert_file = File::open(&tls.certificate_path).await?;
                let key_file = File::open(&tls.pkey_path).await?;

                let cert_file = &mut BufReader::new(cert_file.into_std().await);
                let key_file = &mut BufReader::new(key_file.into_std().await);

                let cert_chain: Vec<Certificate> = certs(cert_file)
                    .filter_map(|v| match v {
                        Ok(cert) => Some(Certificate(cert.to_vec())),
                        Err(e) => {
                            error!("Error loading certificate: {e}");
                            None
                        }
                    })
                    .collect();
                let keys: Vec<PrivatePkcs8KeyDer<'static>> = pkcs8_private_keys(key_file)
                    .filter_map(|v| match v {
                        Ok(pk) => Some(pk),
                        Err(e) => {
                            error!("Error with private key: {e}");
                            None
                        }
                    })
                    .collect();

                let key = keys.first().ok_or(anyhow!("No key found..."))?.to_owned();

                axum_server::bind_rustls(
                    opts.web_addr,
                    tls_config(&cert_chain, key.secret_pkcs8_der()).await?,
                )
                .serve(router.into_make_service())
                .await?;
            }
        }

        Ok(())
    }
}

async fn handle_webapp(
    State(state): State<RouterState>,
    request: Request<Body>,
) -> impl IntoResponse {
    let mut response = ServeDir::new(&state.html_dir)
        .append_index_html_on_directories(true)
        .not_found_service(ServeFile::new(state.html_dir.join("index.html")))
        .try_call(request)
        .await
        .unwrap();

    // 404 pages are handled directly by SPA own router.
    // We just return the default index.html file as 200 OK
    if response.status() == StatusCode::NOT_FOUND {
        *response.status_mut() = StatusCode::OK;
    }

    response
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
