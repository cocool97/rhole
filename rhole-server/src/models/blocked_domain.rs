use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, SimpleObject)]
pub struct BlockedDomain {
    pub domain_id: i32,
    pub domain_address: String,
    pub insert_timestamp: f64,
    pub blocked_count: i32,
    pub whitelisted: bool,
}

impl From<entity::blocked_domains::Model> for BlockedDomain {
    fn from(value: entity::blocked_domains::Model) -> Self {
        Self {
            domain_id: value.id,
            domain_address: value.domain_address,
            insert_timestamp: value.insert_timestamp,
            blocked_count: value.blocked_count,
            whitelisted: value.whitelisted,
        }
    }
}
