use crate::config::config::Settings;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(_cfg: &Settings) {
    let f = std::env::var("RUST_LOG").unwrap_or_else(|_| "info,actix_web=info".into());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(f))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

