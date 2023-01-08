use actix_web::{web::Data, HttpResponse, Responder};
use anyhow::Result;

use crate::{api::models::Client, models::AppData};

use super::not_found::internal_server_error;

pub async fn clients(data: Data<AppData>) -> impl Responder {
    match _clients(data).await {
        Ok(c) => HttpResponse::Ok().json(&c),
        Err(e) => internal_server_error(e.to_string()),
    }
}

pub async fn _clients(data: Data<AppData>) -> Result<Vec<Client>> {
    data.database_controller.get_clients().await
}
