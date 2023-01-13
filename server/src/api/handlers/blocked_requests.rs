use crate::{api::models::BlockedRequest, models::AppData};
use actix_web::{
    web::{Data, Query},
    HttpResponse, Responder,
};
use anyhow::Result;
use serde::Deserialize;

use super::not_found::internal_server_error;

#[derive(Deserialize)]
pub struct QueryInfo {
    limit: u32,
}

pub async fn blocked_requests(
    data: Data<AppData>,
    query: Option<Query<QueryInfo>>,
) -> impl Responder {
    match _blocked_requests(data, query).await {
        Ok(v) => HttpResponse::Ok().json(&v),
        Err(e) => internal_server_error(e.to_string()),
    }
}

pub async fn _blocked_requests(
    data: Data<AppData>,
    query: Option<Query<QueryInfo>>,
) -> Result<Vec<BlockedRequest>> {
    data.database_controller
        .get_blocked_requests(query.map(|v| v.limit))
        .await
}
