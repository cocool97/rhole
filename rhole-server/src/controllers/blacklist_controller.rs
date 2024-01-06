use crate::models::SourceEntry;
use crate::utils;
use crate::{api_models::BlockedRequest, models::SourceType};
use anyhow::{anyhow, Result};
use regex::RegexBuilder;
use reqwest::Url;
use std::convert::TryFrom;

use super::{DatabaseController, NetworkController, WatcherController};

#[derive(Clone)]
pub struct BlacklistController {
    db_controller: DatabaseController,
    blocked_requests_controller: WatcherController<Option<BlockedRequest>, i32>,
}

impl BlacklistController {
    pub fn new(
        db_controller: DatabaseController,
        blocked_requests_controller: WatcherController<Option<BlockedRequest>, i32>,
    ) -> Self {
        Self {
            db_controller,
            blocked_requests_controller,
        }
    }

    pub async fn init_from_sources(&self, sources: &[SourceEntry]) -> Result<()> {
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

            match self
                .db_controller
                .init_blocked_domains(blacklisted_domains)
                .await
            {
                Ok(entries_added) => {
                    log::info!("Initialization finished...");
                    log::info!("Found {} more addresses to blacklist...", entries_added);
                }
                Err(e) => log::error!("{}", e),
            }
        }

        Ok(())
    }

    pub async fn is_domain_blacklisted<I>(&self, domain_address: I) -> Result<Option<i32>>
    where
        I: IntoIterator<Item = String>,
    {
        self.db_controller
            .is_domain_blacklisted(domain_address)
            .await
    }

    pub async fn notify_blocked(&self, blocked_request: &BlockedRequest, client_id: i32) {
        self.blocked_requests_controller
            .notify(Some(blocked_request.to_owned()), Some(client_id))
            .await;
    }
}
