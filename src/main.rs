#![forbid(unsafe_code)]

mod controllers;
mod models;
mod utils;

use crate::controllers::RequestsController;
use crate::models::{Config, RholeCommand};

use anyhow::Result;
use clap::Parser;
use controllers::BlacklistController;
use models::Opts;
use tokio::net::UdpSocket;
use trust_dns_server::ServerFuture;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.command {
        RholeCommand::Start { debug, config_path } => {
            utils::set_log_level(debug);

            log::info!("Starting...");

            let config = Config::from_file(config_path).await?;

            let blacklist_controller =
                BlacklistController::init_from_sources(config.sources.entries, config.database)
                    .await?;

            let socket =
                UdpSocket::bind((config.net.listen_addr.as_str(), config.net.listen_port)).await?;

            let mut server = ServerFuture::new(
                RequestsController::new(
                    blacklist_controller.get_blacklist(),
                    config.proxy_server.clone(),
                )
                .await?,
            );
            server.register_socket(socket);
            server.block_until_done().await?;
        }
        RholeCommand::Info => {
            println!("-- Build --");
            println!("Build Timestamp:\t{}", env!("VERGEN_BUILD_TIMESTAMP"));
            println!("Build Version:\t\t{}\n", env!("VERGEN_BUILD_SEMVER"));

            println!("-- GIT --");
            println!("Commit SHA:\t\t{}", env!("VERGEN_GIT_SHA"));
            println!("Commit Date:\t\t{}", env!("VERGEN_GIT_COMMIT_TIMESTAMP"));
            println!("Commit Branch:\t\t{}\n", env!("VERGEN_GIT_BRANCH"));

            println!("-- rustc --");
            println!("rustc Version:\t\t{}", env!("VERGEN_RUSTC_SEMVER"));
            println!("rustc Channel:\t\t{}", env!("VERGEN_RUSTC_CHANNEL"));
            println!("rustc Host Triple:\t{}", env!("VERGEN_RUSTC_HOST_TRIPLE"));
            println!("rustc Commit SHA\t{}\n", env!("VERGEN_RUSTC_COMMIT_HASH"));

            println!("-- cargo --");
            println!(
                "cargo Target Triple:\t{}",
                env!("VERGEN_CARGO_TARGET_TRIPLE")
            );
            println!("cargo Profile:\t\t{}", env!("VERGEN_CARGO_PROFILE"));
            println!("cargo features:\t\t{}\n", env!("VERGEN_CARGO_FEATURES"));

            println!("-- Build System Informations --");
            println!("OS Version:\t\t{}", env!("VERGEN_SYSINFO_OS_VERSION"));
            println!("CPU Vendor:\t\t{}", env!("VERGEN_SYSINFO_CPU_VENDOR"));
        }
    }

    Ok(())
}
