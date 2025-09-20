// src/middleware/cache.rs placeholder
use actix_web::http::{header, StatusCode};
use actix_web::{HttpRequest, HttpResponse};

use chrono::{DateTime, Utc};
use httpdate::{fmt_http_date, parse_http_date};
use std::time::{SystemTime, UNIX_EPOCH};

/// Gợi ý cache cho response; gắn vào header và dùng cho Conditional GET.
#[derive(Clone, Debug, Default)]
pub struct CacheHint {
    pub etag: Option<String>,               // strong ETag (ví dụ "sha256:xxxx")
    pub last_modified: Option<DateTime<Utc>>,
    pub max_age_seconds: Option<u32>,       // phát Cache-Control: max-age=...
    pub is_public: bool,                    // public hay private
}

/// Áp dụng header cache từ CacheHint vào HttpResponse builder.
pub fn apply_cache_headers(mut resp: HttpResponse, hint: &CacheHint) -> HttpResponse {
    if let Some(etag) = &hint.etag {
        resp.headers_mut().insert(
            header::ETAG,
            header::HeaderValue::from_str(etag).unwrap_or_else(|_| header::HeaderValue::from_static("")),
        );
    }
    if let Some(lm) = &hint.last_modified {
        let s = lm.to_rfc2822(); // convert to HTTP-date later
        let http_date = httpdate::fmt_http_date(lm.naive_utc().and_utc().into());
        resp.headers_mut().insert(
            header::LAST_MODIFIED,
            header::HeaderValue::from_str(&http_date).unwrap(),
        );
    }
    // Cache-Control
    if let Some(ttl) = hint.max_age_seconds {
        let dir = if hint.is_public { "public" } else { "private" };
        let v = format!("{}, max-age={}", dir, ttl);
        resp.headers_mut().insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_str(&v).unwrap(),
        );
    }
    resp
}

/// Kiểm tra If-None-Match / If-Modified-Since và trả 304 nếu thoả.
pub fn maybe_not_modified(req: &HttpRequest, hint: &CacheHint) -> Option<HttpResponse> {
    // Prefer ETag
    if let Some(etag) = &hint.etag {
        if let Some(inm) = req.headers().get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok()) {
            if inm.split(',').map(|s| s.trim()).any(|tag| tag == etag) {
                return Some(HttpResponse::NotModified().finish());
            }
        }
    }
    // Fallback to Last-Modified
    if let Some(lm) = &hint.last_modified {
        if let Some(ims) = req.headers().get(header::IF_MODIFIED_SINCE).and_then(|v| v.to_str().ok()) {
            if let Ok(when) = parse_http_date(ims) {
                // if resource not modified since ims => 304
                let lm_ts = lm.timestamp();
                let ims_ts = when.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
                if lm_ts <= ims_ts {
                    return Some(HttpResponse::NotModified().finish());
                }
            }
        }
    }
    None
}
