use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Client::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Client::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Client::Address)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Client::Alias).string().null())
                    .col(ColumnDef::new(Client::LastSeen).float().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Client::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Client {
    Table,
    Id,
    #[sea_orm(iden = "address")]
    Address,
    #[sea_orm(iden = "alias")]
    Alias,
    #[sea_orm(iden = "last_seen")]
    LastSeen,
}
