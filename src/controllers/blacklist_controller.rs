use std::{collections::HashSet, convert::TryFrom};

use regex::RegexBuilder;
use reqwest::Url;

use super::NetworkController;

pub struct BlacklistController {
    blacklist: HashSet<String>,
}

impl BlacklistController {
    pub async fn init_from_sources(sources: Vec<String>) -> Self {
        log::debug!("Received {} source(s)...", sources.len());

        let network_controller = NetworkController::new();
        let mut blacklist = HashSet::new();

        let regex = RegexBuilder::new(".*\\s(?P<address>\\S*)")
            .swap_greed(false)
            .build()
            .unwrap();

        for source in sources {
            let content = network_controller
                .get(Url::try_from(source.as_str()).unwrap())
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            for line in content.lines() {
                if line.starts_with('#') || line.is_empty() {
                    // Skipping comments
                    continue;
                }
                let address = regex
                    .captures(line)
                    .unwrap()
                    .name("address")
                    .unwrap()
                    .as_str();
                blacklist.insert(address.into());
            }
        }

        log::debug!("Found {} addresses to blacklist...", blacklist.len());

        Self { blacklist }
    }

    pub fn get_blacklist(self) -> HashSet<String> {
        self.blacklist
    }
}
