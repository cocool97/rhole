use std::convert::TryFrom;

use anyhow::{anyhow, Result};
use regex::RegexBuilder;
use reqwest::Url;
use sled::Db;

use crate::models::{DatabaseConfig, SourceEntry, SourceType};

use super::NetworkController;

pub struct BlacklistController {
    blacklist: Db,
}

impl BlacklistController {
    pub async fn init_from_sources(
        sources: Vec<SourceEntry>,
        db_config: DatabaseConfig,
    ) -> Result<Self> {
        log::debug!("Received {} source(s)...", sources.len());

        let network_controller = NetworkController::new();

        // TODO: Configure creation
        // TODO: Add a trait for cache
        let blacklist = sled::open(db_config.path)?;

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

                // Addresses are added in reversed form
                // e.g fr.facebook.com would be added as com.facebook.fr
                // Like that we can iterate over each component of domain name and check if it'll be contained
                let mut rev_address = String::new();
                for component in address.split('.').rev() {
                    rev_address.push_str(component);
                    rev_address.push('.');
                }
                rev_address.pop();

                blacklist.insert(rev_address, true.to_string().as_bytes())?;
            }
        }

        log::info!("Initialization finished...");
        log::info!("Found {} addresses to blacklist...", blacklist.len());

        Ok(Self { blacklist })
    }

    pub fn get_blacklist(self) -> Db {
        self.blacklist
    }
}
