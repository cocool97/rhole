use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    net::{AddrParseError, Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub database: DatabaseConfig,
    pub web_resources: WebResources,
    pub net: NetConfig,
    pub proxy_server: ProxyServer,
    pub sources: Sources,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub internal: PathBuf,
    pub stats: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebResources {
    pub static_files: PathBuf,
    pub mount_path: String,
    pub index_file: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NetConfig {
    pub dns: NetDnsConfig,
    pub web_interface: NetWebInterfaceConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NetDnsConfig {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NetWebInterfaceConfig {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProxyServer {
    pub addr: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sources {
    pub update_interval: u64,
    pub entries: Vec<SourceEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SourceEntry {
    pub source_type: SourceType,
    pub location: String,
    pub comment: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum SourceType {
    Network,
    File,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cache {
    pub validity: u16,
}

impl Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            SourceType::Network => "Network",
            SourceType::File => "File",
        };

        write!(f, "{val}")
    }
}

impl TryInto<SocketAddr> for ProxyServer {
    type Error = AddrParseError;

    fn try_into(self) -> Result<SocketAddr, Self::Error> {
        Ok(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::from_str(&self.addr)?,
            self.port,
        )))
    }
}
