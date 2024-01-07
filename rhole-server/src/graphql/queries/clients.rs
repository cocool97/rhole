use std::net::SocketAddr;

use crate::models::{Client, GraphQLState};
use anyhow::{anyhow, bail, Result};
use async_graphql::{Context, Object};
use log::error;

#[derive(Default)]
pub struct ClientsQuery;

#[Object]
impl ClientsQuery {
    pub async fn clients<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Client>> {
        match ctx.data::<GraphQLState>() {
            Ok(app_data) => app_data.database_controller.get_clients().await,
            Err(e) => {
                error!("{}", e.message);
                Err(anyhow!("{e:?}"))
            }
        }
    }

    pub async fn get_own_client_id<'ctx>(&self, ctx: &Context<'ctx>) -> Result<i32> {
        let ip = match ctx.data::<SocketAddr>() {
            Ok(app_data) => app_data.ip().to_string(),
            Err(e) => {
                error!("{}", e.message);
                bail!("{e:?}")
            }
        };

        let app_data = match ctx.data::<GraphQLState>() {
            Ok(app_data) => app_data,
            Err(e) => {
                error!("{}", e.message);
                bail!("{e:?}")
            }
        };

        let client = app_data
            .database_controller
            .get_client_from_addr(&ip)
            .await?
            .ok_or(anyhow!("Could not find client with address {ip}"))?;

        Ok(client.client_id)
    }
}
