use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};
use log::error;

use crate::models::GraphQLState;

#[derive(Default)]
pub struct BlockedDomainsMutation;

#[Object]
impl BlockedDomainsMutation {
    pub async fn set_domain_whitelist_status<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        domain_id: i32,
        whitelisted: bool,
    ) -> Result<bool> {
        match ctx.data::<GraphQLState>() {
            Ok(app_data) => {
                app_data
                    .database_controller
                    .set_domain_whitelist_status(domain_id, whitelisted)
                    .await?;

                log::info!("Set whitelist={whitelisted} for domain {domain_id}");
                Ok(whitelisted)
            }
            Err(e) => {
                error!("{}", e.message);
                Err(anyhow!("{e:?}"))
            }
        }
    }
}
