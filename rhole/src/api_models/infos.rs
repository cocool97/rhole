use std::time::Duration;

use async_graphql::SimpleObject;
use humantime::format_duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, SimpleObject)]
pub struct Infos {
    pub uptime: String,

    pub build_version: &'static str,
    pub build_timestamp: &'static str,

    pub git_commit_sha: &'static str,
    pub git_commit_date: &'static str,
    pub git_commit_branch: &'static str,

    pub rustc_version: &'static str,
    pub rustc_channel: &'static str,
    pub rustc_host_triple: &'static str,
    pub rustc_commit_sha: &'static str,

    pub cargo_target: &'static str,
    pub cargo_debug: &'static str,
    pub cargo_features: &'static str,

    pub build_os_version: &'static str,
    pub build_cpu_vendor: &'static str,
}

impl Infos {
    pub fn new(duration: Duration) -> Self {
        Self {
            uptime: format_duration(duration).to_string(),
            build_version: env!("CARGO_PKG_VERSION"),
            build_timestamp: env!("VERGEN_BUILD_TIMESTAMP"),
            git_commit_sha: env!("VERGEN_GIT_SHA"),
            git_commit_date: env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
            git_commit_branch: env!("VERGEN_GIT_BRANCH"),
            rustc_version: env!("VERGEN_RUSTC_SEMVER"),
            rustc_channel: env!("VERGEN_RUSTC_CHANNEL"),
            rustc_host_triple: env!("VERGEN_RUSTC_HOST_TRIPLE"),
            rustc_commit_sha: env!("VERGEN_RUSTC_COMMIT_HASH"),
            build_os_version: env!("VERGEN_SYSINFO_OS_VERSION"),
            build_cpu_vendor: env!("VERGEN_SYSINFO_CPU_VENDOR"),
            cargo_target: env!("VERGEN_CARGO_TARGET_TRIPLE"),
            cargo_debug: env!("VERGEN_CARGO_DEBUG"),
            cargo_features: env!("VERGEN_CARGO_FEATURES"),
        }
    }
}
