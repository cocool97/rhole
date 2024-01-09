use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};
use log::error;

use crate::models::GraphQLState;

#[derive(Default)]
pub struct ClientsMutation;

#[Object]
impl ClientsMutation {
    pub async fn set_client_alias<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        client_id: i32,
        alias: String,
    ) -> Result<bool> {
        match ctx.data::<GraphQLState>() {
            Ok(app_data) => {
                app_data
                    .database_controller
                    .set_client_alias(client_id, alias.clone())
                    .await?;

                log::info!("Set alias={alias} for client {client_id}");
                Ok(true)
            }
            Err(e) => {
                error!("{}", e.message);
                Err(anyhow!("{e:?}"))
            }
        }
    }
}
