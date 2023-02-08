use std::{convert::TryFrom, net::IpAddr};

use anyhow::{anyhow, Result};
use common::{SourceEntry, SourceType};
use regex::RegexBuilder;
use reqwest::Url;
use sqlx::sqlite::SqliteQueryResult;

use crate::utils;

use super::{DatabaseController, NetworkController};

pub struct BlacklistController {
    db_controller: DatabaseController,
}

impl BlacklistController {
    pub async fn init_from_sources(
        sources: &[SourceEntry],
        db_controller: DatabaseController,
    ) -> Result<Self> {
        log::debug!("Received {} source(s)...", sources.len());

        let network_controller = NetworkController::new();

        let regex = RegexBuilder::new(".*\\s(?P<address>\\S*)")
            .swap_greed(false)
            .build()?;

        log::info!("Starting blacklist insertion...");
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
                SourceType::File => tokio::fs::read_to_string(&source.location).await?,
            };

            let mut blacklisted_domains = vec![];
            for line in content.lines() {
                if line.starts_with('#') || line.is_empty() {
                    // Skipping comments
                    continue;
                }
                let address = regex
                    .captures(line)
                    .and_then(|v| v.name("address").map(|address| address.as_str()))
                    .ok_or_else(|| anyhow!("line does not match parsing regex..."))?;

                let rev_address = utils::reverse_domain_name(address);

                blacklisted_domains.push(rev_address);
            }

            if let Ok(entries_added) = db_controller.add_blocked_domains(blacklisted_domains).await
            {
                log::info!("Initialization finished...");
                log::info!("Found {} addresses to blacklist...", entries_added);
            }
        }

        Ok(Self { db_controller })
    }

    pub async fn is_domain_blacklisted<S: AsRef<str>>(
        &self,
        domain_address: S,
    ) -> Result<Option<u32>> {
        self.db_controller
            .is_domain_blacklisted(domain_address)
            .await
    }

    pub async fn add_blocked_request(
        &self,
        client_ip: IpAddr,
        domain_id: u32,
    ) -> Result<SqliteQueryResult> {
        self.db_controller
            .add_blocked_request(client_ip, domain_id)
            .await
    }
}
