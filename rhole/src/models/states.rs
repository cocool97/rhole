use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::api_models::ServerConfig;
use crate::controllers::DatabaseController;
use crate::graphql::RholeQueries;
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Clone)]
pub struct RouterState {
    pub router_data: Arc<RouterData>,
}

pub struct RouterData {
    pub graphql_schema: Schema<RholeQueries, EmptyMutation, EmptySubscription>,
}

pub struct GraphQLState {
    pub config: ServerConfig,
    pub database_controller: DatabaseController,
    pub start_time: SystemTime,
}
