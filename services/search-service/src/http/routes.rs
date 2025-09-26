use actix_web::{web, Scope};
use crate::http::handlers;
// permissions applied at gateway or future middleware

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(handlers::health::healthz))
        .service(handlers::search_all::search)
        .service(handlers::reindex::reindex)
        .service(handlers::sync_test::sync_test)
}
