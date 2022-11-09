use std::env;

pub fn set_log_level(debug: bool) {
    // Setting RUST_LOG from env if it exists
    // If it does not exist, creates special one
    if env::var("RUST_LOG").is_err() {
        let level = if debug { "debug" } else { "info" };
        std::env::set_var("RUST_LOG", level);
    }

    env_logger::init();
}
