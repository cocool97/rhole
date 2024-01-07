use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, SimpleObject)]
pub struct LiveRequest {
    pub request_id: u16,
    pub client_address: String,
    pub client_id: i32,
    pub request_address: String,
    pub timestamp: f64,
}
