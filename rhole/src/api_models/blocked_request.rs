use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, SimpleObject)]
pub struct BlockedRequest {
    pub request_id: u32,
    pub client_id: u32,
    pub request_address: String,
    pub timestamp: f64,
}
