use anyhow::Result;
use vergen::{Config, ShaKind};

fn main() -> Result<()> {
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Normal;
    *config.git_mut().enabled_mut() = true;
    *config.git_mut().semver_mut() = true;

    vergen::vergen(config)
}
