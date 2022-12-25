use std::{
    net::IpAddr,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteQueryResult},
    Pool, Row, Sqlite,
};

pub struct DatabaseController {
    pool: Pool<Sqlite>,
}

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

            // sqlx::query(
            //     r#"
            //     CREATE TABLE IF NOT EXISTS monthly_stats (
            //         month_id INTEGER PRIMARY KEY AUTOINCREMENT,
            //         month_name STRING,
            //         client_id INTEGER,
            //         blocked_requests INTEGER,
            //         FOREIGN KEY(client_id) REFERENCES clients(client_id)
            //     );"#,
            // )
            // .execute(&mut conn)
            // .await?;

            // sqlx::query(
            //     r#"
            //     CREATE TABLE IF NOT EXISTS yearly_stats (
            //         year_id INTEGER PRIMARY KEY AUTOINCREMENT,
            //         year_name STRING,
            //         client_id INTEGER,
            //         blocked_requests INTEGER,
            //         FOREIGN KEY(client_id) REFERENCES clients(client_id)
            //     );"#,
            // )
            // .execute(&mut conn)
            // .await?;
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
