use actix_web::{web, Scope};

pub fn api_scope() -> Scope {
    web::scope("")
        .service(crate::http::handlers::health::healthz)
        .service(crate::http::handlers::upload::presign_upload)
        .service(crate::http::handlers::download::presign_download)
        .service(crate::http::handlers::sign::attach)
}
