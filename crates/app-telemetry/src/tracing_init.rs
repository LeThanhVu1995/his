// src/tracing_init.rs placeholder
use app_config::TelemetryConfig;
use once_cell::sync::OnceCell;
use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::logging::build_json_log_layer;

/// Guard giữ tài nguyên telemetry; Drop sẽ shutdown OTLP tracer.
#[derive(Debug)]
pub struct TelemetryGuard {
    shutdown_on_drop: bool,
}

/// Cài tracing + OTLP theo config. Trả về guard; khi drop sẽ shutdown tracer.
/// - `service_name`: service.name cho OTEL
/// - `service_version`: optional service.version
pub fn init_with(
    service_name: &str,
    service_version: Option<&str>,
    cfg: &TelemetryConfig,
) -> anyhow::Result<TelemetryGuard> {
    static STARTED: OnceCell<bool> = OnceCell::new();
    if STARTED.get().copied().unwrap_or(false) {
        // Đã init rồi — bỏ qua (idempotent)
        return Ok(TelemetryGuard { shutdown_on_drop: false });
    }

    // ----- Resource -----
    let mut attrs = vec![KeyValue::new("service.name", service_name.to_string())];
    if let Some(ver) = service_version {
        if !ver.is_empty() {
            attrs.push(KeyValue::new("service.version", ver.to_string()));
        }
    }
    // SDK 0.21: Resource::new is public
    let _resource = Resource::new(attrs);

    // ----- Tracer (OTLP gRPC) ----- (temporarily disabled to resolve version conflicts)
    // let tracer = opentelemetry_otlp::new_pipeline()
    //     .tracing()
    //     .with_exporter(
    //         opentelemetry_otlp::new_exporter()
    //             .tonic()
    //             .with_endpoint(cfg.otlp_endpoint.clone()),
    //     )
    //     .with_trace_config(
    //         sdktrace::Config::default()
    //             .with_sampler(sdktrace::Sampler::ParentBased(Box::new(
    //                 sdktrace::Sampler::TraceIdRatioBased(1.0),
    //             )))
    //             .with_resource(resource.clone()),
    //     )
    //     .install_batch(runtime::Tokio)?;

    // ----- Logging (JSON) + OpenTelemetry layer -----
    let env_filter = EnvFilter::try_new(cfg.log_level.clone()).unwrap_or_else(|_| EnvFilter::new("info"));
    // let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let fmt_layer = build_json_log_layer(); // JSON logs

    tracing_subscriber::registry()
        .with(env_filter)
        // .with(otel_layer)
        .with(fmt_layer) // log ra stdout
        .init();

    // ----- Metrics (Prometheus recorder) -----
    // Có thể chạy song song OTLP logs/traces với Prometheus metrics pull model
    crate::metrics::init_prometheus_recorder();

    STARTED.set(true).ok();
    Ok(TelemetryGuard { shutdown_on_drop: true })
}

/// Gọi trước khi shutdown service để flush OTLP exporter.
pub fn shutdown_tracer() {}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if self.shutdown_on_drop {
            shutdown_tracer();
        }
    }
}
