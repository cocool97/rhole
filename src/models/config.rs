use std::path::Path;

use anyhow::Result;
use serde::Deserialize;
use tokio::fs::File;

#[derive(Deserialize)]
pub struct Config {
    pub net: NetConfig,
}

#[derive(Deserialize)]
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
