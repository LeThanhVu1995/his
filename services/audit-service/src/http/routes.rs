use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(crate::http::handlers::health::healthz))
        .service(
            web::resource("/api/v1/audit/events")
                .wrap(RequirePermission::new(crate::security::policy::perm::AUDIT_READ))
                .route(web::get().to(crate::http::handlers::list::list))
        )
        .service(
            web::resource("/api/v1/audit/events/by-actor")
                .wrap(RequirePermission::new(crate::security::policy::perm::AUDIT_READ))
                .route(web::get().to(crate::http::handlers::by_actor::by_actor))
        )
        .service(
            web::resource("/api/v1/audit/events/by-entity")
                .wrap(RequirePermission::new(crate::security::policy::perm::AUDIT_READ))
                .route(web::get().to(crate::http::handlers::by_entity::by_entity))
        )
        .service(
            web::resource("/api/v1/audit/events:export")
                .wrap(RequirePermission::new(crate::security::policy::perm::AUDIT_EXPORT))
                .route(web::get().to(crate::http::handlers::export::export_ndjson))
        )
        .service(
            web::resource("/internal/audit:write")
                .wrap(RequirePermission::new(crate::security::policy::perm::AUDIT_WRITE))
                .route(web::post().to(crate::http::handlers::write::write))
        )
}
