use crate::models::AppData;
use actix_web::{
    web::{Data, Query},
    HttpResponse, Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryInfo {
    limit: u32,
}

pub async fn blocked_requests(
    data: Data<AppData>,
    query: Option<Query<QueryInfo>>,
) -> impl Responder {
    let blocked_requests = data
        .database_controller
        .get_blocked_requests(query.map(|v| v.limit))
        .await
        .unwrap();

    HttpResponse::Ok().json(blocked_requests)
}
