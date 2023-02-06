use crate::controllers::DatabaseController;
use common::ServerConfig;
use std::time::SystemTime;

pub struct AppData {
    pub config: ServerConfig,
    pub database_controller: DatabaseController,
    pub start_time: SystemTime,
}
