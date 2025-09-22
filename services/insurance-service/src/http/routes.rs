use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;
use crate::security::policy::perm;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(crate::http::handlers::health::healthz))
        // Eligibility
        .service(
            web::resource("/api/v1/ins/eligibility:check")
                .wrap(RequirePermission::new(perm::ELIGIBILITY_CHECK))
                .route(web::post().to(crate::http::handlers::eligibility::check::check))
        )
        // Claims
        .service(
            web::resource("/api/v1/ins/claims:create")
                .wrap(RequirePermission::new(perm::CLAIM_CREATE))
                .route(web::post().to(crate::http::handlers::claims::create::create))
        )
        .service(
            web::resource("/api/v1/ins/claims/{id}")
                .wrap(RequirePermission::new(perm::CLAIM_GET))
                .route(web::get().to(crate::http::handlers::claims::get::get))
        )
        .service(
            web::resource("/api/v1/ins/claims/{id}:submit")
                .wrap(RequirePermission::new(perm::CLAIM_SUBMIT))
                .route(web::post().to(crate::http::handlers::claims::submit::submit))
        )
        .service(
            web::resource("/api/v1/ins/claims/{id}:sign")
                .wrap(RequirePermission::new(perm::CLAIM_SIGN))
                .route(web::post().to(crate::http::handlers::claims::sign::sign))
        )
        .service(
            web::resource("/api/v1/ins/claims/{id}:status/{st}")
                .wrap(RequirePermission::new(perm::CLAIM_STATUS))
                .route(web::post().to(crate::http::handlers::claims::status::set_status))
        )
        // Reconciliation
        .service(
            web::resource("/api/v1/ins/reconciliations:create")
                .wrap(RequirePermission::new(perm::RECON_CREATE))
                .route(web::post().to(crate::http::handlers::reconciliations::create::create))
        )
}
