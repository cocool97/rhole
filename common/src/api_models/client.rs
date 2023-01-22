use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    pub client_id: u32,
    pub address: String,
    pub last_seen: f64,
}
