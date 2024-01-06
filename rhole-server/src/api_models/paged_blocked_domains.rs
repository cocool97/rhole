use crate::models::BlockedDomain;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, SimpleObject)]
pub struct PagedBlockedDomains {
    pub blocked_domains: Vec<BlockedDomain>,
    pub total_row_count: u64,
}

impl PagedBlockedDomains {
    pub fn new(blocked_domains: Vec<BlockedDomain>, total_row_count: u64) -> Self {
        Self {
            blocked_domains,
            total_row_count,
        }
    }
}
