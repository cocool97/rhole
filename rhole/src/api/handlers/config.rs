use crate::models::AppData;
use actix_web::{web::Data, HttpResponse, Responder};

pub async fn config(data: Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(&data.config)
}
