use super::RholeClient;
use anyhow::Result;
use common::ServerConfig;

impl RholeClient {
    pub async fn config(&self) -> Result<ServerConfig> {
        Ok(self
            .client
            .get(format!("{}/config", self.url))
            .send()
            .await?
            .json()
            .await?)
    }
}
