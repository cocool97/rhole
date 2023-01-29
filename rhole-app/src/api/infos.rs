use common::Infos;

use super::RholeClient;
use anyhow::Result;

impl RholeClient {
    pub async fn infos(&self) -> Result<Infos> {
        Ok(self
            .client
            .get(format!("{}/infos", self.url))
            .send()
            .await?
            .json()
            .await?)
    }
}
