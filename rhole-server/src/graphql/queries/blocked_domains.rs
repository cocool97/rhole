use crate::models::{BlockedDomain, GraphQLState};
use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};
use log::error;

#[derive(Default)]
pub struct BlockedDomainsQuery;

#[Object]
impl BlockedDomainsQuery {
    pub async fn blocked_domains<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<BlockedDomain>> {
        match ctx.data::<GraphQLState>() {
            Ok(app_data) => {
                app_data
                    .database_controller
                    .get_blocked_domains(Some(1024))
                    .await
            }
            Err(e) => {
                error!("{}", e.message);
                Err(anyhow!("{e:?}"))
            }
        }
    }
}
