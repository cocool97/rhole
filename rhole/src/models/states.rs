use async_graphql::{EmptyMutation, Schema};

use crate::api_models::{BlockedRequest, ServerConfig};
use crate::controllers::{DatabaseController, WatcherController};
use crate::graphql::{RholeQueries, RholeSubscriptions};
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Clone)]
pub struct RouterState {
    pub router_data: Arc<RouterData>,
}

pub struct RouterData {
    pub graphql_schema: Schema<RholeQueries, EmptyMutation, RholeSubscriptions>,
}

pub struct GraphQLState {
    pub config: ServerConfig,
    pub database_controller: DatabaseController,
    pub start_time: SystemTime,
    pub blocked_requests_controller: WatcherController<Option<BlockedRequest>>,
}
