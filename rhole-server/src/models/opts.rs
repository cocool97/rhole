use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Parser)]
#[command(author, about, long_about)]
pub struct Opts {
    /// Enables debug mode
    #[clap(short = 'd', long = "debug", env = "DEBUG")]
    pub debug: bool,
    /// Path to server configuration file
    #[clap(short = 'c', long = "config", env = "CONFIG_PATH")]
    pub config_path: PathBuf,
    /// DNS server listening address
    #[clap(long = "dns-addr", env = "DNS_ADDR", default_value = "0.0.0.0:53")]
    pub dns_addr: SocketAddr,
    /// Web interface listening address
    #[clap(long = "web-addr", env = "WEB_ADDR", default_value = "0.0.0.0:443")]
    pub web_addr: SocketAddr,
    /// Do not update database from sources
    #[clap(short = 'n', long = "no-update-db")]
    pub no_update_db: bool,
}
