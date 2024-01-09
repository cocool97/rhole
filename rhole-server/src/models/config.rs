use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{
    collections::HashMap,
    fmt::Display,
    net::{AddrParseError, Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub tls: Option<TlsConfig>,
    #[serde(default)]
    pub local_hosts: HashMap<String, Ipv4Addr>,
    pub proxy_server: ProxyServer,
    pub sources: Sources,
    #[serde(default, flatten)]
    pub extra_args: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TlsConfig {
    pub certificate_path: String,
    pub pkey_path: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProxyServer {
    pub ip: String,
    pub port: u16,
    pub tls_dns_name: String,
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq)]
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
            Ipv4Addr::from_str(&self.ip)?,
            self.port,
        )))
    }
}
