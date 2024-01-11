//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "blocked_requests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub client_id: i32,
    pub domain_id: i32,
    #[sea_orm(column_type = "Double")]
    pub blocked_timestamp: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::blocked_domains::Entity",
        from = "Column::DomainId",
        to = "super::blocked_domains::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    BlockedDomains,
    #[sea_orm(
        belongs_to = "super::client::Entity",
        from = "Column::ClientId",
        to = "super::client::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Client,
}

impl Related<super::blocked_domains::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BlockedDomains.def()
    }
}

impl Related<super::client::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Client.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}