pub fn info() {
    println!("-- Build --");
    println!("Build Timestamp:\t{}", env!("VERGEN_BUILD_TIMESTAMP"));
    println!("Build Version:\t\t{}\n", env!("VERGEN_BUILD_SEMVER"));

    println!("-- GIT --");
    println!("Commit SHA:\t\t{}", env!("VERGEN_GIT_SHA"));
    println!("Commit Date:\t\t{}", env!("VERGEN_GIT_COMMIT_TIMESTAMP"));
    println!("Commit Branch:\t\t{}\n", env!("VERGEN_GIT_BRANCH"));

    println!("-- rustc --");
    println!("rustc Version:\t\t{}", env!("VERGEN_RUSTC_SEMVER"));
    println!("rustc Channel:\t\t{}", env!("VERGEN_RUSTC_CHANNEL"));
    println!("rustc Host Triple:\t{}", env!("VERGEN_RUSTC_HOST_TRIPLE"));
    println!("rustc Commit SHA\t{}\n", env!("VERGEN_RUSTC_COMMIT_HASH"));

    println!("-- cargo --");
    println!(
        "cargo Target Triple:\t{}",
        env!("VERGEN_CARGO_TARGET_TRIPLE")
    );
    println!("cargo Profile:\t\t{}", env!("VERGEN_CARGO_PROFILE"));
    println!("cargo features:\t\t{}\n", env!("VERGEN_CARGO_FEATURES"));

    println!("-- Build System Informations --");
    println!("OS Version:\t\t{}", env!("VERGEN_SYSINFO_OS_VERSION"));
    println!("CPU Vendor:\t\t{}", env!("VERGEN_SYSINFO_CPU_VENDOR"));
}
