use anyhow::Result;
use vergen::EmitBuilder;

fn main() -> Result<()> {
    EmitBuilder::builder()
        .build_timestamp()
        .git_sha(false)
        .git_branch()
        .git_commit_timestamp()
        .cargo_features()
        .cargo_target_triple()
        .cargo_debug()
        .sysinfo_os_version()
        .sysinfo_cpu_vendor()
        .rustc_commit_hash()
        .rustc_host_triple()
        .rustc_channel()
        .rustc_semver()
        .emit()
}
