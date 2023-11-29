use anyhow::Result;
use futures::TryStreamExt;
use sqlx::{
    sqlite::{
        SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteQueryResult,
        SqliteSynchronous,
    },
    ConnectOptions, Connection, Pool, Row, Sqlite,
};
use std::{
    net::IpAddr,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::api_models::{BlockedDomain, BlockedRequest, Client};

#[derive(Clone)]
pub struct DatabaseController {
    pool: Pool<Sqlite>,
}

impl DatabaseController {
    pub async fn init_database<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        let options = SqliteConnectOptions::default()
            .create_if_missing(true)
            .filename(database_path)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .foreign_keys(true)
            .disable_statement_logging();

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
                    ip_address STRING UNIQUE,
                    last_seen REAL
                );"#,
            )
            .execute(&mut *conn)
            .await?;

            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS blocked_requests (
                    request_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    client_id INTEGER,
                    domain_id INTEGER,
                    blocked_timestamp REAL,
                    FOREIGN KEY(client_id) REFERENCES clients(client_id),
                    FOREIGN KEY(domain_id) REFERENCES blocked_domains(domain_id) 
                );"#,
            )
            .execute(&mut *conn)
            .await?;

            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS blocked_domains (
                    domain_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    domain_address STRING UNIQUE,
                    insert_timestamp REAL,
                    whitelisted INTEGER
                );"#,
            )
            .execute(&mut *conn)
            .await?;

            sqlx::query(
                r#"
                CREATE INDEX IF NOT EXISTS blocked_domains_idx ON blocked_domains(domain_address, whitelisted);
                "#,
            )
            .execute(&mut *conn)
            .await?;
        }

        Ok(Self { pool })
    }

    pub async fn add_blocked_domains(&self, blocked_domains: Vec<String>) -> Result<u32> {
        let mut conn = self.pool.acquire().await?;

        let mut tr = conn.begin().await?;

        let mut entries_added = 0;
        for blocked_domain in blocked_domains {
            // TODO: Utc or Date ?
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

            match sqlx::query(
                r#"INSERT OR IGNORE INTO blocked_domains (domain_address, insert_timestamp, whitelisted) VALUES (?, ?, ?);"#,
            )
            .bind(blocked_domain)
            .bind(timestamp)
            .bind(0)
            .execute(&mut *tr)
            .await {
                Ok(_) => entries_added += 1,
                Err(e) => log::error!("Error when inserting: {e}"),
            };
        }

        tr.commit().await?;

        Ok(entries_added)
    }

    pub async fn is_domain_blacklisted<S: AsRef<str>>(&self, domain: S) -> Result<Option<u32>> {
        let mut conn = self.pool.acquire().await?;

        let row = sqlx::query(
            r#"SELECT domain_id FROM blocked_domains WHERE domain_address = ? AND whitelisted = 0;
        "#,
        )
        .bind(domain.as_ref())
        .fetch_one(&mut *conn)
        .await?;

        match row.try_get("domain_id") {
            Ok(id) => Ok(Some(id)),
            Err(_) => Ok(None),
        }
    }

    pub async fn add_blocked_request(
        &self,
        client_address: IpAddr,
        domain_id: u32,
    ) -> Result<SqliteQueryResult> {
        self._upsert_client_informations(client_address).await?;
        self._add_blocked_request(client_address, domain_id).await
    }

    pub async fn get_blocked_requests(&self, num: Option<u32>) -> Result<Vec<BlockedRequest>> {
        let mut conn = self.pool.acquire().await?;

        // TODO: change hardcoded value of 1024
        let num_entries = num.unwrap_or(1024);

        let mut rows = sqlx::query(
            r#"SELECT request_id, client_id, blocked_timestamp, domain_address
            FROM blocked_requests 
            LEFT JOIN blocked_domains ON blocked_requests.domain_id = blocked_domains.domain_id 
            ORDER BY blocked_timestamp 
            DESC LIMIT ?;
        "#,
        )
        .bind(num_entries)
        .fetch(&mut *conn);

        let mut res = vec![];
        while let Some(row) = rows.try_next().await? {
            res.push(BlockedRequest {
                request_id: row.try_get("request_id")?,
                client_id: row.try_get("client_id")?,
                request_address: row.try_get("domain_address")?,
                timestamp: row.try_get("blocked_timestamp")?,
            })
        }

        Ok(res)
    }

    pub async fn get_blocked_domains(&self, num: Option<u32>) -> Result<Vec<BlockedDomain>> {
        let mut conn = self.pool.acquire().await?;

        // TODO: change hardcoded value of 1024
        let num_entries = num.unwrap_or(1024);

        let mut rows = sqlx::query(
            r#"SELECT A.*, COUNT(B.domain_id) AS blocked_count
            FROM blocked_domains AS A
            LEFT JOIN blocked_requests AS B ON B.domain_id = A.domain_id
            GROUP BY A.domain_id
            LIMIT ?;
        "#,
        )
        .bind(num_entries)
        .fetch(&mut *conn);

        let mut res = vec![];
        while let Some(row) = rows.try_next().await? {
            res.push(BlockedDomain {
                domain_id: row.try_get("domain_id")?,
                domain_address: row.try_get("domain_address")?,
                insert_timestamp: row.try_get("insert_timestamp")?,
                blocked_count: row.try_get("blocked_count")?,
                whitelisted: row.try_get("whitelisted")?,
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
        .fetch(&mut *conn);

        let mut res = vec![];
        while let Some(row) = rows.try_next().await? {
            res.push(Client {
                client_id: row.try_get("client_id")?,
                address: row.try_get("ip_address")?,
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
            r#"INSERT INTO clients (ip_address, last_seen) VALUES (?, ?)
            ON CONFLICT(ip_address) DO UPDATE SET last_seen=?;
        "#,
        )
        .bind(client_address.to_string())
        .bind(timestamp)
        .bind(timestamp)
        .execute(&mut *conn)
        .await?)
    }

    async fn _add_blocked_request(
        &self,
        client_address: IpAddr,
        domain_id: u32,
    ) -> Result<SqliteQueryResult> {
        let mut conn = self.pool.acquire().await?;

        // TODO: Utc or Date ?
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

        // Get client_id
        let client_id: i64 =
            sqlx::query(r#"SELECT client_id FROM clients WHERE ip_address = ? LIMIT 1;"#)
                .bind(client_address.to_string())
                .fetch_one(&mut *conn)
                .await?
                .try_get("client_id")?;

        Ok(sqlx::query(
            r#"INSERT INTO blocked_requests (client_id, domain_id, blocked_timestamp) 
            VALUES (?, ?, ?);
        "#,
        )
        .bind(client_id)
        .bind(domain_id)
        .bind(timestamp)
        .execute(&mut *conn)
        .await?)
    }
}
