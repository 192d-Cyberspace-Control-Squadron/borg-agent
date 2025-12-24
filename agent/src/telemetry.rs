pub fn init() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info,agent=info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}
