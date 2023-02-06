use std::time::SystemTime;

use crate::models::AppData;
use actix_web::{web::Data, HttpResponse, Responder};
use anyhow::Result;
use common::Infos;
use humantime::format_duration;

use super::not_found::internal_server_error;

pub async fn infos(data: Data<AppData>) -> impl Responder {
    match _infos(data).await {
        Ok(i) => HttpResponse::Ok().json(&i),
        Err(e) => internal_server_error(e.to_string()),
    }
}

async fn _infos(data: Data<AppData>) -> Result<Infos> {
    let formatted_duration = SystemTime::now().duration_since(data.start_time)?;

    Ok(Infos {
        uptime: format_duration(formatted_duration).to_string(),
        build_version: env!("VERGEN_BUILD_SEMVER").to_string(),
        build_timestamp: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
        git_commit_sha: env!("VERGEN_GIT_SHA").to_string(),
        git_commit_date: env!("VERGEN_GIT_COMMIT_TIMESTAMP").to_string(),
        git_commit_branch: env!("VERGEN_GIT_BRANCH").to_string(),
        rustc_version: env!("VERGEN_RUSTC_SEMVER").to_string(),
        rustc_channel: env!("VERGEN_RUSTC_CHANNEL").to_string(),
        rustc_host_triple: env!("VERGEN_RUSTC_HOST_TRIPLE").to_string(),
        rustc_commit_sha: env!("VERGEN_RUSTC_COMMIT_HASH").to_string(),
        build_os_version: env!("VERGEN_SYSINFO_OS_VERSION").to_string(),
        build_cpu_vendor: env!("VERGEN_SYSINFO_CPU_VENDOR").to_string(),
        cargo_target: env!("VERGEN_CARGO_TARGET_TRIPLE").to_string(),
        cargo_profile: env!("VERGEN_CARGO_PROFILE").to_string(),
        cargo_features: env!("VERGEN_CARGO_FEATURES").to_string(),
    })
}
