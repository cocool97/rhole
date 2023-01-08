use serde::Serialize;
use sqlx::{sqlite::SqliteRow, Row};

#[derive(Debug, Serialize)]
pub struct Client {
    pub client_id: u32,
    pub address: String,
    pub last_seen: f64,
}

impl TryFrom<SqliteRow> for Client {
    type Error = sqlx::Error;

    fn try_from(value: SqliteRow) -> Result<Self, Self::Error> {
        Ok(Client {
            client_id: value.try_get("client_id")?,
            address: value.try_get("address")?,
            last_seen: value.try_get("last_seen")?,
        })
    }
}
