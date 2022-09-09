use anyhow::Result;
use reqwest::{Client, Response, Url};

pub struct NetworkController {
    client: Client,
}

impl NetworkController {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get(&self, url: Url) -> Result<Response> {
        Ok(self.client.get(url).send().await?)
    }
}
