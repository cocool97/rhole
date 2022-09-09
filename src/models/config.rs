use std::path::Path;

use anyhow::Result;
use serde::Deserialize;
use tokio::fs::File;

// TODO: URL instead of String for sources
#[derive(Deserialize)]
pub struct Config {
    pub net: NetConfig,
    pub proxy_server: String,
    pub sources: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct NetConfig {
    pub listen_addr: String,
    pub listen_port: u16,
}

impl Config {
    pub async fn from_file<P: AsRef<Path>>(p: P) -> Result<Self> {
        let f = File::open(p).await?;
        Ok(serde_yaml::from_reader(f.into_std().await)?)
    }
}
