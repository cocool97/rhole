use anyhow::Result;
use futures::executor::block_on;
use migration::{Migrator, MigratorTrait};
use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    ConnectOptions,
};
use std::path::Path;

const SQLDB_MIN_CONNECTIONS: u32 = 10;
const SQLDB_MAX_CONNECTIONS: u32 = 1000;

// Some sea-orm-cli commands to :
// - Generate a new migration: `sea-orm-cli migrate generate --universal-time MIGRATION_NAME`
// - (Re)generate entities: `sea-orm-cli generate entity --database-url sqlite://rhole.db --output-dir entity/src/entities`

#[derive(Clone)]
pub struct DatabaseController {
    pub(crate) connection: DatabaseConnection,
}

impl DatabaseController {
    pub async fn init_database<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        // Need to go from sqlx::* to create database if not existing.
        // sea_orm does not provide this ability.
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

        let connection = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);

        // Executes needed migrations before everything else
        block_on(Migrator::up(&connection, None))?;

        Ok(Self { connection })
    }
}
