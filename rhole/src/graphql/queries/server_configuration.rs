use crate::{api_models::ServerConfig, models::GraphQLState};
use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};
use log::error;

#[derive(Default)]
pub struct ServerConfigurationQuery;

#[Object]
impl ServerConfigurationQuery {
    pub async fn server_configuration<'ctx>(&self, ctx: &Context<'ctx>) -> Result<ServerConfig> {
        match ctx.data::<GraphQLState>() {
            Ok(app_data) => Ok(app_data.config.clone()),
            Err(e) => {
                error!("{}", e.message);
                Err(anyhow!("{e:?}"))
            }
        }
    }
}
