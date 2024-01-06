use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BlockedDomains::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlockedDomains::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BlockedDomains::DomainAddress)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlockedDomains::InsertTimestamp)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlockedDomains::BlockedCount)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BlockedDomains::Whitelisted)
                            .boolean()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BlockedDomains::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BlockedDomains {
    Table,
    Id,
    #[sea_orm(iden = "domain_address")]
    DomainAddress,
    #[sea_orm(iden = "insert_timestamp")]
    InsertTimestamp,
    #[sea_orm(iden = "blocked_count")]
    BlockedCount,
    #[sea_orm(iden = "whitelisted")]
    Whitelisted,
}
