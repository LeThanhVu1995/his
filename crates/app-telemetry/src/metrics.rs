use once_cell::sync::OnceCell;
use std::time::Duration;

use metrics::{counter, describe_counter, describe_gauge, describe_histogram, gauge, histogram};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

// KHÔNG cần opentelemetry_api; thay bằng:
use opentelemetry::trace::TraceContextExt;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Lưu PrometheusHandle toàn cục để render /metrics
static PROM_HANDLE: OnceCell<PrometheusHandle> = OnceCell::new();

pub fn init_prometheus_recorder() {
    if PROM_HANDLE.get().is_some() {
        return;
    }

    let builder = PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_server_request_duration_ms".to_string()),
            &[
                1.0, 2.0, 5.0, 10.0, 20.0, 30.0, 50.0, 75.0, 100.0,
                150.0, 200.0, 300.0, 400.0, 500.0,
                750.0, 1000.0, 1500.0, 2000.0, 3000.0, 5000.0,
                8000.0, 10000.0, 15000.0, 30000.0, 60000.0,
            ],
        ).expect("set buckets for metric");

    let handle = builder.install_recorder().expect("install prometheus recorder");
    PROM_HANDLE.set(handle).ok();

    describe_counter!("http_server_requests_total", "Total number of HTTP requests received");
    describe_histogram!("http_server_request_duration_ms", "HTTP server request duration (ms)");
    describe_gauge!("http_server_requests_in_flight", "Number of HTTP requests currently in flight");
}

pub fn record_http_request(method: &str, path: &str, status: u16, elapsed: Duration) {
    // let method = method.to_uppercase();
    // let status = status.to_string();
    // let route = normalize_route(path);

    // Use metrics without labels to avoid 'static lifetime requirements for label values.
    let ctr = counter!("http_server_requests_total");
    ctr.increment(1);

    let hist = histogram!("http_server_request_duration_ms");
    hist.record(elapsed.as_secs_f64() * 1_000.0);
}

pub fn record_http_inflight_delta(delta: i64) {
    gauge!("http_server_requests_in_flight").increment(delta as f64);
}

fn normalize_route(path: &str) -> String {
    path.split_once('?').map(|(p, _)| p.to_string()).unwrap_or_else(|| path.to_string())
}

pub fn prometheus_text() -> String {
    PROM_HANDLE.get().map(|h| h.render()).unwrap_or_else(|| "# no prometheus recorder installed\n".to_string())
}

#[cfg(feature = "actix")]
use actix_web::{HttpResponse, Responder};

#[cfg(feature = "actix")]
pub async fn prometheus_handler() -> impl Responder {
    let txt = prometheus_text();
    HttpResponse::Ok().content_type("text/plain; version=0.0.4").body(txt)
}

pub type TraceId = String;

pub fn trace_id() -> Option<TraceId> {
    let span = tracing::Span::current();
    let cx = span.context(); // từ tracing_opentelemetry::OpenTelemetrySpanExt
    let sc = cx.span().span_context().clone(); // cần trait TraceContextExt từ `opentelemetry`
    if sc.is_valid() { Some(sc.trace_id().to_string()) } else { None }
}
