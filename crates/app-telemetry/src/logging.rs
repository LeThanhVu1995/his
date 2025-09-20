// src/logging.rs placeholder
use tracing_subscriber::{
    fmt::{self, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

pub fn build_json_log_layer<S>() -> impl Layer<S>
where
    S: for<'a> tracing_subscriber::registry::LookupSpan<'a> + tracing::Subscriber,
{
    fmt::layer()
        .event_format(
            fmt::format()
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .with_target(true)
                .with_level(true)
                .with_file(false)
                .with_line_number(false)
                .with_timer(UtcTime::rfc_3339()),
        )
        .with_writer(std::io::stdout)
}

pub fn init_logger_json(default_level: &str) {
    let env_filter = tracing_subscriber::EnvFilter::try_new(default_level)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(build_json_log_layer())
        .init();
}
