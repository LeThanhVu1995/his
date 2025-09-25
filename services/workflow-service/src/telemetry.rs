use crate::config::Settings;

pub fn init_tracing(cfg: &Settings) {
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "workflow_service=debug,actix_web=info".to_string());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}
