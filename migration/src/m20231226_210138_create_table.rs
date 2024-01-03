use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_create_table::Client, m20231226_200350_create_table::BlockedDomains};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BlockedRequests::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlockedRequests::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BlockedRequests::ClientId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlockedRequests::DomainId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlockedRequests::BlockedTimestamp)
                            .float()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("client-id-fk")
                            .from(BlockedRequests::Table, BlockedRequests::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_delete(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("domain-id-fk")
                            .from(BlockedRequests::Table, BlockedRequests::DomainId)
                            .to(BlockedDomains::Table, BlockedDomains::Id)
                            .on_delete(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BlockedRequests::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BlockedRequests {
    Table,
    Id,
    #[sea_orm(iden = "client_id")]
    ClientId,
    #[sea_orm(iden = "domain_id")]
    DomainId,
    #[sea_orm(iden = "blocked_timestamp")]
    BlockedTimestamp,
}
