use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Infos {
    pub uptime: String,

    pub build_version: String,
    pub build_timestamp: String,

    pub git_commit_sha: String,
    pub git_commit_date: String,
    pub git_commit_branch: String,

    pub rustc_version: String,
    pub rustc_channel: String,
    pub rustc_host_triple: String,
    pub rustc_commit_sha: String,

    pub cargo_target: String,
    pub cargo_profile: String,
    pub cargo_features: String,

    pub build_os_version: String,
    pub build_cpu_vendor: String,
}
