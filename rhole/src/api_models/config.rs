use anyhow::Result;
use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    net::{AddrParseError, Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct ServerConfig {
    pub database_path: String,
    pub tls: TlsConfig,
    pub html_dir: String,
    #[serde(default)]
    pub local_hosts: HashMap<String, Ipv4Addr>,
    pub net: NetConfig,
    pub proxy_server: ProxyServer,
    pub sources: Sources,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct TlsConfig {
    pub certificate_path: String,
    pub pkey_path: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct NetConfig {
    pub dns: NetDnsConfig,
    pub dot: NetDotConfig,
    pub web_interface: NetWebInterfaceConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct NetDnsConfig {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct NetDotConfig {
    pub listen_addr: String,
    pub listen_port: u16,
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct NetWebInterfaceConfig {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct ProxyServer {
    pub ip: String,
    pub port: u16,
    pub tls_dns_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct Sources {
    pub update_interval: u64,
    pub entries: Vec<SourceEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct SourceEntry {
    pub source_type: SourceType,
    pub location: String,
    pub comment: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Enum)]
pub enum SourceType {
    Network,
    File,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
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
            Ipv4Addr::from_str(&self.ip)?,
            self.port,
        )))
    }
}
