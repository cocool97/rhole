use std::time::SystemTime;

use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};

use crate::{api_models::Infos, models::AppData};

#[derive(Default)]
pub struct InfosQuery;

#[Object]
impl InfosQuery {
    pub async fn infos<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Infos> {
        let app_data = ctx.data::<AppData>().map_err(|e| anyhow!("{e:?}"))?;
        let duration = SystemTime::now().duration_since(app_data.start_time)?;
        Ok(Infos::new(duration))
    }
}
