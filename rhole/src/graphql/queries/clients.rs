use crate::api_models::client::Client;
use crate::models::AppData;
use anyhow::{anyhow, Result};
use async_graphql::{Context, Object};
use log::error;

#[derive(Default)]
pub struct ClientsQuery;

#[Object]
impl ClientsQuery {
    pub async fn clients<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Client>> {
        match ctx.data::<AppData>() {
            Ok(app_data) => app_data.database_controller.get_clients().await,
            Err(e) => {
                error!("{}", e.message);
                Err(anyhow!("{e:?}"))
            }
        }
    }
}
