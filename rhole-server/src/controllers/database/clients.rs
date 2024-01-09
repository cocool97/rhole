use std::{
    net::IpAddr,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::models::Client;

use super::DatabaseController;
use anyhow::Result;
use entity::client::ActiveModel as ClientActiveModel;
use entity::client::Column as ClientColumn;
use entity::client::Entity as ClientEntity;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

impl DatabaseController {
    pub async fn upsert_client(&self, client_address: IpAddr) -> Result<Client> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

        match ClientEntity::find()
            .filter(ClientColumn::Address.eq(client_address.to_string()))
            .one(&self.connection)
            .await?
        {
            Some(client) => {
                // Into ActiveModel
                let mut client: ClientActiveModel = client.into();
                client.last_seen = ActiveValue::Set(now);
                let c = client.update(&self.connection).await?;

                Ok(Client::from(c))
            }
            None => {
                let client = ClientEntity::insert(ClientActiveModel {
                    id: ActiveValue::NotSet,
                    address: ActiveValue::Set(client_address.to_string()),
                    last_seen: ActiveValue::Set(now),
                    alias: ActiveValue::NotSet,
                })
                .exec_with_returning(&self.connection)
                .await?;

                Ok(Client::from(client))
            }
        }
    }

    pub async fn get_clients(&self) -> Result<Vec<Client>> {
        let clients = ClientEntity::find().all(&self.connection).await?;

        Ok(clients.into_iter().map(Into::into).collect())
    }

    pub async fn get_client_from_addr<S: ToString>(&self, addr: S) -> Result<Option<Client>> {
        Ok(ClientEntity::find()
            .filter(ClientColumn::Address.eq(addr.to_string()))
            .one(&self.connection)
            .await
            .map(|m| m.map(Into::into))?)
    }

    pub async fn set_client_alias(&self, client_id: i32, alias: String) -> Result<()> {
        let model = ClientActiveModel {
            id: ActiveValue::Unchanged(client_id),
            address: ActiveValue::NotSet,
            alias: ActiveValue::Set(Some(alias)),
            last_seen: ActiveValue::NotSet,
        };
        let _ = ClientEntity::update(model)
            .filter(ClientColumn::Id.eq(client_id))
            .exec(&self.connection)
            .await?;

        Ok(())
    }
}
