use anyhow::Result;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::{controllers::DatabaseController, models::ServerConfig};

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, SimpleObject)]
pub struct BlacklistInfos {
    count: u64,
    total: i64,
    nb_sources: usize,
}

impl BlacklistInfos {
    pub async fn new(
        database_controller: &DatabaseController,
        config: ServerConfig,
    ) -> Result<Self> {
        Ok(Self {
            count: database_controller
                .get_blocked_domains_entries_count()
                .await?,
            total: database_controller.get_blocked_domains_sum().await?,
            nb_sources: config.sources.entries.len(),
        })
    }
}
