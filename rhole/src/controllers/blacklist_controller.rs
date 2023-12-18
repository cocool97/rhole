use crate::models::SourceEntry;
use crate::utils;
use crate::{api_models::BlockedRequest, models::SourceType};
use anyhow::{anyhow, Result};
use regex::RegexBuilder;
use reqwest::Url;
use std::{convert::TryFrom, net::IpAddr};

use super::{DatabaseController, NetworkController, WatcherController};

pub struct BlacklistController {
    db_controller: DatabaseController,
    blocked_requests_controller: WatcherController<Option<BlockedRequest>>,
}

impl BlacklistController {
    pub fn new(
        db_controller: DatabaseController,
        blocked_requests_controller: WatcherController<Option<BlockedRequest>>,
    ) -> Self {
        Self {
            db_controller,
            blocked_requests_controller,
        }
    }

    pub async fn init_from_sources(
        sources: &[SourceEntry],
        db_controller: DatabaseController,
        blocked_requests_controller: WatcherController<Option<BlockedRequest>>,
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

        Ok(Self {
            db_controller,
            blocked_requests_controller,
        })
    }

    pub async fn is_domain_blacklisted<S: AsRef<str>>(
        &self,
        domain_address: S,
    ) -> Result<Option<u32>> {
        self.db_controller
            .is_domain_blacklisted(domain_address)
            .await
    }

    pub async fn add_blocked_request(&self, client_ip: IpAddr, domain_id: u32) -> Result<()> {
        // Insert it in database for future work
        let blocked_request = self
            .db_controller
            .add_blocked_request(client_ip, domain_id)
            .await?;

        // Notify all watchers that a new domain has been blocked
        self.blocked_requests_controller
            .notify(Some(blocked_request))
            .await;

        Ok(())
    }
}
