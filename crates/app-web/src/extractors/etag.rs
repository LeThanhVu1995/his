// src/extractors/etag.rs placeholder
use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::header;
use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};
use httpdate::fmt_http_date;
use base64::Engine;

/// Tạo strong ETag từ bytes: "sha256:<base64>"
pub fn make_strong_etag(bytes: &[u8]) -> String {
    let hash = Sha256::digest(bytes);
    let b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(hash);
    format!("\"sha256:{}\"", b64)
}

/// Format HTTP-date từ chrono DateTime<Utc>
pub fn http_date_from(dt: DateTime<Utc>) -> String {
    fmt_http_date(dt.naive_utc().and_utc().into())
}

/// Kiểm tra If-None-Match nhanh (phù hợp API trả JSON nhỏ).
/// Nếu thỏa điều kiện => trả 304 luôn, ngược lại trả None để handler tiếp tục.
pub fn check_conditional_etag(req: &HttpRequest, etag: &str) -> Option<HttpResponse> {
    if let Some(inm) = req.headers().get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok()) {
        if inm.split(',').map(|s| s.trim()).any(|tag| tag == etag) {
            return Some(HttpResponse::NotModified().finish());
        }
    }
    None
}
