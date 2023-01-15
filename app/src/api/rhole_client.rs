use anyhow::{anyhow, Result};
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
            url: format!(
                "{}/api",
                web_sys::window()
                    .ok_or_else(|| anyhow!("Could not get window..."))?
                    .location()
                    .origin()
                    .map_err(|_| anyhow!("Could not get origin for window..."))?
            ),
        })
    }
}
