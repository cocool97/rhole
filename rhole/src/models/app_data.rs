use std::time::SystemTime;

use crate::controllers::DatabaseController;

pub struct AppData {
    pub database_controller: DatabaseController,
    pub start_time: SystemTime,
}
