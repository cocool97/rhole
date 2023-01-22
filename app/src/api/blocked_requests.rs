use super::RholeClient;
use anyhow::Result;
use common::BlockedRequest;

impl RholeClient {
    pub async fn blocked_requests(&self) -> Result<Vec<BlockedRequest>> {
        Ok(self
            .client
            .get(format!("{}/blocked?limit=100", self.url))
            .send()
            .await?
            .json()
            .await?)
    }
}
