use std::{fmt::Display, io, path::Path};

use anyhow::Result;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use tokio::fs::File;

fn check_path_exists<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let location: String = Deserialize::deserialize(deserializer)?;
    if Path::new(&location).exists() {
        Ok(location)
    } else {
        Err(D::Error::custom(io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No such file or directory: '{}'", location),
        )))
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub net: NetConfig,
    pub proxy_server: ProxyServer,
    pub sources: Sources,
}

#[derive(Clone, Deserialize)]
pub struct NetConfig {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Deserialize)]
pub struct ProxyServer {
    pub addr: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Sources {
    pub update_interval: u64,
    pub entries: Vec<SourceEntry>
}

#[derive(Clone, Debug, Deserialize)]
pub struct SourceEntry {
    pub source_type: SourceType,
    #[serde(deserialize_with = "check_path_exists")]
    pub location: String,
    pub comment: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum SourceType {
    Network,
    File,
}

#[derive(Clone, Deserialize)]
pub struct Cache {
    pub validity: u16,
}

impl Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            SourceType::Network => "Network",
            SourceType::File => "File",
        };

        write!(f, "{}", val)
    }
}

impl ProxyServer {
    pub fn to_addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}

impl Config {
    pub async fn from_file<P: AsRef<Path>>(p: P) -> Result<Self> {
        let f = File::open(p).await?;
        Ok(serde_yaml::from_reader(f.into_std().await)?)
    }
}
