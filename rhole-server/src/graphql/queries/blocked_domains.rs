use crate::{api_models::PagedBlockedDomains, models::GraphQLState};
use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};
use log::{error, trace};

#[derive(Default)]
pub struct BlockedDomainsQuery;

#[Object]
impl BlockedDomainsQuery {
    pub async fn paged_blocked_domains<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        page: i32,
        page_size: i32,
    ) -> Result<PagedBlockedDomains> {
        trace!("Requesting page {page} / page_size {page_size}");
        match ctx.data::<GraphQLState>() {
            Ok(app_data) => {
                let blocked_domains = app_data
                    .database_controller
                    .get_blocked_domains(page, page_size)
                    .await?;

                let total_row_count = app_data
                    .database_controller
                    .get_blocked_domains_entries_count()
                    .await?;

                Ok(PagedBlockedDomains::new(blocked_domains, total_row_count))
            }
            Err(e) => {
                error!("{}", e.message);
                Err(anyhow!("{e:?}"))
            }
        }
    }
}
