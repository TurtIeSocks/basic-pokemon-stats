fn main() {
    dotenv::from_filename(std::env::var("ENV").unwrap_or(".env".to_string())).ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info".to_string()));

    if let Err(err) = api::start() {
        log::error!("[INIT] Unable to start API: {:?}", err)
    }
}
