use crate::api_models::BlockedRequest;
use crate::models::AppData;
use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};
use log::error;

#[derive(Default)]
pub struct BlockedRequestsQuery;

#[Object]
impl BlockedRequestsQuery {
    pub async fn blocked_requests<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<BlockedRequest>> {
        match ctx.data::<AppData>() {
            Ok(app_data) => {
                app_data
                    .database_controller
                    .get_blocked_requests(None)
                    .await
            }
            Err(e) => {
                error!("{}", e.message);
                Err(anyhow!("{e:?}"))
            }
        }
    }
}
