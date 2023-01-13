use std::time::SystemTime;

use crate::{api::models::Infos, models::AppData};
use actix_web::{web::Data, HttpResponse, Responder};
use anyhow::Result;
use humantime::format_duration;

use super::not_found::internal_server_error;

pub async fn infos(data: Data<AppData>) -> impl Responder {
    match _infos(data).await {
        Ok(i) => HttpResponse::Ok().json(&i),
        Err(e) => internal_server_error(e.to_string()),
    }
}

pub async fn _infos(data: Data<AppData>) -> Result<Infos> {
    let formatted_duration = SystemTime::now().duration_since(data.start_time)?;

    Ok(Infos {
        uptime: format_duration(formatted_duration).to_string(),
        version: env!("VERGEN_BUILD_SEMVER").to_string(),
    })
}
