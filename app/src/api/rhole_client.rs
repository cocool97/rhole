use anyhow::Result;
use reqwest::{Client, ClientBuilder};

pub struct RholeClient {
    pub(crate) client: Client,
    pub(crate) url: String,
}

impl RholeClient {
    pub fn new() -> Result<Self> {
        let client = ClientBuilder::new().build()?;
        Ok(Self {
            client,
            // TODO: Change it !
            url: "http://127.0.0.1:8080/api".to_string(),
        })
    }
}
