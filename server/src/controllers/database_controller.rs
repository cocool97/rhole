use anyhow::Result;
use common::{BlockedRequest, Client};
use futures::TryStreamExt;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteQueryResult},
    Pool, Row, Sqlite,
};
use std::{
    net::IpAddr,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone)]
pub struct DatabaseController {
    pool: Pool<Sqlite>,
}

// TODO: Enable journal mode ?

impl DatabaseController {
    pub async fn init_database<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        let options = SqliteConnectOptions::default()
            .create_if_missing(true)
            .filename(database_path)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        {
            let mut conn = pool.acquire().await?;

            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS clients (
                    client_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    address STRING UNIQUE,
                    last_seen REAL
                );"#,
            )
            .execute(&mut conn)
            .await?;

            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS blocked_requests (
                    request_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    client_id INTEGER,
                    request_address STRING,
                    timestamp REAL,
                    FOREIGN KEY(client_id) REFERENCES clients(client_id)
                );"#,
            )
            .execute(&mut conn)
            .await?;
        }

        Ok(Self { pool })
    }

    pub async fn add_blocked_request(
        &self,
        client_address: IpAddr,
        blocked_request: String,
    ) -> Result<SqliteQueryResult> {
        self._upsert_client_informations(client_address).await?;
        self._add_blocked_request(client_address, blocked_request)
            .await
    }

    pub async fn get_blocked_requests(&self, num: Option<u32>) -> Result<Vec<BlockedRequest>> {
        let mut conn = self.pool.acquire().await?;

        // TODO: change hardcoded value of 1024
        let num_entries = num.unwrap_or(1024);

        let mut rows = sqlx::query(
            r#"SELECT * FROM blocked_requests ORDER BY timestamp DESC LIMIT ?;
        "#,
        )
        .bind(num_entries)
        .fetch(&mut conn);

        let mut res = vec![];
        while let Some(row) = rows.try_next().await? {
            res.push(BlockedRequest {
                request_id: row.try_get("request_id")?,
                client_id: row.try_get("client_id")?,
                request_address: row.try_get("request_address")?,
                timestamp: row.try_get("timestamp")?,
            })
        }

        Ok(res)
    }

    pub async fn get_clients(&self) -> Result<Vec<Client>> {
        let mut conn = self.pool.acquire().await?;

        let mut rows = sqlx::query(
            r#"SELECT * FROM clients ORDER BY last_seen;
        "#,
        )
        .fetch(&mut conn);

        let mut res = vec![];
        while let Some(row) = rows.try_next().await? {
            res.push(Client {
                client_id: row.try_get("client_id")?,
                address: row.try_get("address")?,
                last_seen: row.try_get("last_seen")?,
            })
        }

        Ok(res)
    }

    async fn _upsert_client_informations(
        &self,
        client_address: IpAddr,
    ) -> Result<SqliteQueryResult> {
        let mut conn = self.pool.acquire().await?;

        // TODO: Utc or Date ?
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

        Ok(sqlx::query(
            r#"INSERT INTO clients (address, last_seen) VALUES (?, ?)
            ON CONFLICT(address) DO UPDATE SET last_seen=?;
        "#,
        )
        .bind(client_address.to_string())
        .bind(timestamp)
        .bind(timestamp)
        .execute(&mut conn)
        .await?)
    }

    async fn _add_blocked_request(
        &self,
        client_address: IpAddr,
        blocked_request: String,
    ) -> Result<SqliteQueryResult> {
        let mut conn = self.pool.acquire().await?;

        // TODO: Utc or Date ?
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

        // Get client_id
        let client_id: i64 =
            sqlx::query(r#"SELECT client_id FROM clients WHERE address = ? LIMIT 1;"#)
                .bind(client_address.to_string())
                .fetch_one(&mut conn)
                .await?
                .try_get("client_id")?;

        Ok(sqlx::query(
            r#"INSERT INTO blocked_requests (client_id, request_address, timestamp) 
            VALUES (?, ?, ?);
        "#,
        )
        .bind(client_id)
        .bind(blocked_request)
        .bind(timestamp)
        .execute(&mut conn)
        .await?)
    }
}
