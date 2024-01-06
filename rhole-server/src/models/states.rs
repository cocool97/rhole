use async_graphql::Schema;

use crate::api_models::{BlockedRequest, LiveRequest};
use crate::controllers::{DatabaseController, WatcherController};
use crate::graphql::{RholeMutations, RholeQueries, RholeSubscriptions};
use std::path::PathBuf;
use std::time::SystemTime;

use super::ServerConfig;

#[derive(Clone)]
pub struct RouterState {
    pub graphql_schema: Schema<RholeQueries, RholeMutations, RholeSubscriptions>,
    pub html_dir: PathBuf,
}

pub struct GraphQLState {
    pub config: ServerConfig,
    pub database_controller: DatabaseController,
    pub start_time: SystemTime,
    pub blocked_requests_controller: WatcherController<Option<BlockedRequest>, i32>,
    pub live_requests_controller: WatcherController<Option<LiveRequest>, i32>,
}
