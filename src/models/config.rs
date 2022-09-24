use std::{fmt::Display, path::Path};

use anyhow::Result;
use serde::Deserialize;
use tokio::fs::File;

#[derive(Deserialize)]
pub struct Config {
    pub net: NetConfig,
    pub proxy_server: String,
    pub sources: Vec<SourceEntry>,
}

#[derive(Clone, Deserialize)]
pub struct NetConfig {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Clone, Deserialize)]
pub struct SourceEntry {
    pub source_type: SourceType,
    pub location: String,
    pub comment: String,
}

#[derive(Clone, Deserialize)]
pub enum SourceType {
    Network,
    File,
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

impl Config {
    pub async fn from_file<P: AsRef<Path>>(p: P) -> Result<Self> {
        let f = File::open(p).await?;
        Ok(serde_yaml::from_reader(f.into_std().await)?)
    }
}
