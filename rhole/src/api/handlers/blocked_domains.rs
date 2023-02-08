use crate::models::AppData;
use actix_web::{
    web::{Data, Query},
    HttpResponse, Responder,
};
use anyhow::Result;
use common::BlockedDomain;
use serde::Deserialize;

use super::not_found::internal_server_error;

#[derive(Deserialize)]
pub struct QueryInfo {
    limit: u32,
}

pub async fn blocked_domains(
    data: Data<AppData>,
    query: Option<Query<QueryInfo>>,
) -> impl Responder {
    match _blocked_domains(data, query).await {
        Ok(v) => HttpResponse::Ok().json(&v),
        Err(e) => internal_server_error(e.to_string()),
    }
}

async fn _blocked_domains(
    data: Data<AppData>,
    query: Option<Query<QueryInfo>>,
) -> Result<Vec<BlockedDomain>> {
    data.database_controller
        .get_blocked_domains(query.map(|v| v.limit))
        .await
}
