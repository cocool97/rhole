use std::{collections::HashSet, convert::TryFrom};

use anyhow::{anyhow, Result};
use regex::RegexBuilder;
use reqwest::Url;

use crate::models::{SourceEntry, SourceType};

use super::NetworkController;

pub struct BlacklistController {
    blacklist: HashSet<String>,
}

impl BlacklistController {
    pub async fn init_from_sources(sources: Vec<SourceEntry>) -> Result<Self> {
        log::debug!("Received {} source(s)...", sources.len());

        let network_controller = NetworkController::new();
        let mut blacklist = HashSet::new();

        let regex = RegexBuilder::new(".*\\s(?P<address>\\S*)")
            .swap_greed(false)
            .build()?;

        for source in sources {
            log::info!(
                "Reading from {}: {} ...",
                source.source_type,
                source.location
            );
            log::debug!("-> {}", source.comment);
            let content = match source.source_type {
                SourceType::Network => {
                    network_controller
                        .get(Url::try_from(source.location.as_str())?)
                        .await?
                        .text()
                        .await?
                }
                SourceType::File => tokio::fs::read_to_string(source.location).await?,
            };

            for line in content.lines() {
                if line.starts_with('#') || line.is_empty() {
                    // Skipping comments
                    continue;
                }
                let address = regex
                    .captures(line)
                    .and_then(|v| v.name("address").map(|address| address.as_str()))
                    .ok_or_else(|| anyhow!("line does not match parsing regex..."))?;
                blacklist.insert(address.into());
            }
        }

        log::debug!("Found {} addresses to blacklist...", blacklist.len());

        Ok(Self { blacklist })
    }

    pub fn get_blacklist(self) -> HashSet<String> {
        self.blacklist
    }
}
