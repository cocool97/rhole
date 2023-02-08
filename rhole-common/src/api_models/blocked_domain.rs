use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub struct BlockedDomain {
    pub domain_id: u32,
    pub domain_address: String,
    pub insert_timestamp: f64,
    pub blocked_count: u32,
    pub whitelisted: bool,
}
