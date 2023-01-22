use super::RholeClient;
use anyhow::Result;
use common::Client;

impl RholeClient {
    pub async fn clients(&self) -> Result<Vec<Client>> {
        Ok(self
            .client
            .get(format!("{}/clients", self.url))
            .send()
            .await?
            .json()
            .await?)
    }
}
