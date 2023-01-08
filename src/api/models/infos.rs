use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Infos {
    pub uptime: String,
    pub version: String,
}
