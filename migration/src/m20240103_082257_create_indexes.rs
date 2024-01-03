use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_create_table::Client, m20231226_200350_create_table::BlockedDomains};

#[derive(DeriveMigrationName)]
pub struct Migration;

const BLOCKED_DOMAINS_INDEX_NAME: &str = "blocked_domains_idx";
const CLIENTS_INDEX_NAME: &str = "clients_idx";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(BLOCKED_DOMAINS_INDEX_NAME)
                    .table(BlockedDomains::Table)
                    .col(BlockedDomains::DomainAddress)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name(CLIENTS_INDEX_NAME)
                    .table(Client::Table)
                    .col(Client::Address)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name(BLOCKED_DOMAINS_INDEX_NAME).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name(CLIENTS_INDEX_NAME).to_owned())
            .await
    }
}
