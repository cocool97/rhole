use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, SimpleObject)]
pub struct Client {
    pub client_id: u32,
    pub address: String,
    pub last_seen: f64,
}
