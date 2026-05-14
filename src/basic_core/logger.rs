pub fn init() {
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "info"); }
    }
    
    pretty_env_logger::init();
    log::info!("Logger has been initialized!");
}
