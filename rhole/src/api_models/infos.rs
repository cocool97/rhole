use std::time::Duration;

use async_graphql::SimpleObject;
use humantime::format_duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, SimpleObject)]
pub struct Infos {
    pub uptime: String,
    pub build_version: &'static str,
}

impl Infos {
    pub fn new(duration: Duration) -> Self {
        Self {
            uptime: format_duration(duration).to_string(),
            build_version: env!("CARGO_PKG_VERSION"),
        }
    }
}
