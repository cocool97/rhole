pub fn set_log_level(debug: bool) {
    let level = if debug { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", level);
    env_logger::init();
}
