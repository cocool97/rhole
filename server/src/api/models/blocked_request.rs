use serde::Serialize;
use sqlx::{sqlite::SqliteRow, Row};

#[derive(Serialize)]
pub struct BlockedRequest {
    pub request_id: u32,
    pub client_id: u32,
    pub request_address: String,
    pub timestamp: f64,
}

impl TryFrom<SqliteRow> for BlockedRequest {
    type Error = sqlx::Error;

    fn try_from(value: SqliteRow) -> Result<Self, Self::Error> {
        Ok(BlockedRequest {
            request_id: value.try_get("request_id")?,
            client_id: value.try_get("client_id")?,
            request_address: value.try_get("request_address")?,
            timestamp: value.try_get("timestamp")?,
        })
    }
}
