use super::RholeClient;
use anyhow::Result;
use common::BlockedDomain;

impl RholeClient {
    pub async fn blocked_domains(&self) -> Result<Vec<BlockedDomain>> {
        Ok(self
            .client
            .get(format!("{}/blocked_domains?limit=1000", self.url))
            .send()
            .await?
            .json()
            .await?)
    }
}
