use std::{
    io::BufReader,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use crate::api_models::ServerConfig;
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    guard,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use anyhow::{anyhow, Result};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::{
    fs::File,
    net::{TcpListener, UdpSocket},
};
use trust_dns_server::ServerFuture;

use crate::{
    controllers::{BlacklistController, DatabaseController, RequestsController},
    graphql::RholeQueries,
    models::AppData,
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

    tokio::spawn(async { server.block_until_done().await });

    let app_data = AppData {
        config: config.clone(),
        database_controller: database_controller.clone(),
        start_time,
    };

    let graphql_schema = Schema::build(RholeQueries::default(), EmptyMutation, EmptySubscription)
        .data(app_data)
        .finish();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            // .allowed_origin("http://localhost:3000")
            // .allowed_methods(vec!["GET", "POST"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            // .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(graphql_schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_graphiql),
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
                            let file =
                                NamedFile::open_async(PathBuf::from(static_files).join(index_file))
                                    .await?;
                            let res = file.into_response(&req);
                            Ok(ServiceResponse::new(req, res))
                        }
                    })
                })
            })
    });

    let listen_addr = (
        config.net.web_interface.listen_addr.as_str(),
        config.net.web_interface.listen_port,
    );
    match http {
        true => {
            server.bind(listen_addr)?.run().await?;
        }
        false => {
            server
                .bind_rustls(listen_addr, tls_config(&cert_chain, &key).await?)?
                .run()
                .await?;
        }
    }

    Ok(())
}

async fn index(
    schema: web::Data<Schema<RholeQueries, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

async fn tls_config(cert_chain: &[Certificate], key: &[u8]) -> Result<rustls::ServerConfig> {
    let mut config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain.to_vec(), PrivateKey(key.to_vec()))?;

    config.alpn_protocols.push(b"http/1.1".to_vec());
    config.alpn_protocols.push(b"h2".to_vec());

    Ok(config)
}
