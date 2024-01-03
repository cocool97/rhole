use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, SimpleObject)]
pub struct Client {
    pub client_id: i32,
    pub address: String,
    pub alias: Option<String>,
    pub last_seen: f64,
}

impl From<entity::client::Model> for Client {
    fn from(value: entity::client::Model) -> Self {
        Self {
            client_id: value.id,
            address: value.address,
            alias: value.alias,
            last_seen: value.last_seen,
        }
    }
}
