use anyhow::Result;
use futures::TryStreamExt;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    ConnectOptions, Pool, Row, Sqlite,
};
use std::{
    net::IpAddr,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::api_models::{BlockedDomain, BlockedRequest, Client};

const SQLDB_MIN_CONNECTIONS: u32 = 10;
const SQLDB_MAX_CONNECTIONS: u32 = 50;

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
            .min_connections(SQLDB_MIN_CONNECTIONS)
            .max_connections(SQLDB_MAX_CONNECTIONS)
            .connect_with(options)
            .await?;

        sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS clients (
                    client_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    ip_address STRING UNIQUE,
                    last_seen REAL
                );"#,
        )
        .execute(&pool)
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
        .execute(&pool)
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
        .execute(&pool)
        .await?;

        sqlx::query(
                r#"
                CREATE INDEX IF NOT EXISTS blocked_domains_idx ON blocked_domains(domain_address, whitelisted);
                "#,
            )
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub async fn add_blocked_domains(&self, blocked_domains: Vec<String>) -> Result<u32> {
        let mut tr = self.pool.begin().await?;

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
        let row = sqlx::query(
            r#"SELECT domain_id FROM blocked_domains WHERE domain_address = ? AND whitelisted = 0;
        "#,
        )
        .bind(domain.as_ref())
        .fetch_one(&self.pool)
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
    ) -> Result<BlockedRequest> {
        self.upsert_client_informations(client_address).await?;
        self._add_blocked_request(client_address, domain_id).await
    }

    pub async fn get_blocked_requests(&self, num: Option<u32>) -> Result<Vec<BlockedRequest>> {
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
        .fetch(&self.pool);

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
        .fetch(&self.pool);

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
        let mut rows = sqlx::query(
            r#"SELECT * FROM clients ORDER BY last_seen;
        "#,
        )
        .fetch(&self.pool);

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

    pub async fn get_blocked_request(&self, request_id: i64) -> Result<BlockedRequest> {
        let row = sqlx::query(
            r#"SELECT A.*, B.*
            FROM blocked_requests AS A
            LEFT JOIN blocked_domains AS B ON B.domain_id = A.domain_id
            WHERE A.request_id = ?;
        "#,
        )
        .bind(request_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(BlockedRequest {
            request_id: row.try_get("request_id")?,
            client_id: row.try_get("client_id")?,
            request_address: row.try_get("domain_address")?,
            timestamp: row.try_get("blocked_timestamp")?,
        })
    }

    pub async fn upsert_client_informations(&self, client_address: IpAddr) -> Result<Client> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

        let row = sqlx::query(
            r#"INSERT INTO clients (ip_address, last_seen) VALUES (?, ?)
            ON CONFLICT(ip_address) DO UPDATE SET last_seen=? RETURNING *;
        "#,
        )
        .bind(client_address.to_string())
        .bind(timestamp)
        .bind(timestamp)
        .fetch_one(&self.pool)
        .await?;

        Ok(Client {
            client_id: row.try_get("client_id")?,
            address: row.try_get("ip_address")?,
            last_seen: row.try_get("last_seen")?,
        })
    }

    async fn _add_blocked_request(
        &self,
        client_address: IpAddr,
        domain_id: u32,
    ) -> Result<BlockedRequest> {
        // TODO: Utc or Date ?
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

        // Get client_id
        let client_id: i64 =
            sqlx::query(r#"SELECT client_id FROM clients WHERE ip_address = ? LIMIT 1;"#)
                .bind(client_address.to_string())
                .fetch_one(&self.pool)
                .await?
                .try_get("client_id")?;

        let request_id = sqlx::query(
            r#"INSERT INTO blocked_requests (client_id, domain_id, blocked_timestamp) 
            VALUES (?, ?, ?);
        "#,
        )
        .bind(client_id)
        .bind(domain_id)
        .bind(timestamp)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.get_blocked_request(request_id).await
    }
}
