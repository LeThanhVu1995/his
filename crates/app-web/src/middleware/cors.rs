// src/middleware/cors.rs placeholder
use actix_cors::Cors;
use actix_web::http;

pub fn build_cors(dev_mode: bool) -> Cors {
    let mut cors = Cors::default()
        .allow_any_header()
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
        .supports_credentials();
    if dev_mode {
        cors = cors.allow_any_origin();
    } else {
        // Tự set domain thật sự của bạn ở prod
        // cors = cors.allowed_origin("https://your.domain.tld");
        cors = cors.allowed_origin_fn(|_origin, _req_head| true); // placeholder
    }
    cors.expose_headers(vec![
        http::header::HeaderName::from_static("x-request-id"),
        http::header::HeaderName::from_static("x-trace-id"),
        http::header::ETAG,
        http::header::LOCATION,
    ])
}
