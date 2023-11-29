use std::time::SystemTime;

use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};

use crate::{api_models::Infos, models::GraphQLState};

#[derive(Default)]
pub struct InfosQuery;

#[Object]
impl InfosQuery {
    pub async fn infos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Infos> {
        let app_state = ctx.data::<GraphQLState>().map_err(|e| anyhow!("{e:?}"))?;
        let duration = SystemTime::now().duration_since(app_state.start_time)?;
        Ok(Infos::new(duration))
    }
}
