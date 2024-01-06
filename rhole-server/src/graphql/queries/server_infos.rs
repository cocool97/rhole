use std::time::SystemTime;

use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};

use crate::{
    api_models::{BlacklistInfos, ServerInfos},
    models::GraphQLState,
};

#[derive(Default)]
pub struct ServerInfosQuery;

#[Object]
impl ServerInfosQuery {
    pub async fn server_infos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<ServerInfos> {
        let app_state = ctx.data::<GraphQLState>().map_err(|e| anyhow!("{e:?}"))?;
        let duration = SystemTime::now().duration_since(app_state.start_time)?;
        Ok(ServerInfos::new(duration))
    }

    pub async fn blacklist_infos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<BlacklistInfos> {
        let app_state = ctx.data::<GraphQLState>().map_err(|e| anyhow!("{e:?}"))?;
        BlacklistInfos::new(&app_state.database_controller, app_state.config.clone()).await
    }
}
