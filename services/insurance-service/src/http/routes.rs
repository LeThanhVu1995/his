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
        // Insurance Payers
        .service(
            web::resource("/api/v1/ins/payers")
                .wrap(RequirePermission::new(perm::PAYER_CREATE))
                .route(web::post().to(crate::http::handlers::ins_payers::create::create_ins_payer))
        )
        .service(
            web::resource("/api/v1/ins/payers")
                .wrap(RequirePermission::new(perm::PAYER_GET))
                .route(web::get().to(crate::http::handlers::ins_payers::list::list_ins_payers))
        )
        .service(
            web::resource("/api/v1/ins/payers/{id}")
                .wrap(RequirePermission::new(perm::PAYER_GET))
                .route(web::get().to(crate::http::handlers::ins_payers::get::get_ins_payer))
        )
        .service(
            web::resource("/api/v1/ins/payers/{id}")
                .wrap(RequirePermission::new(perm::PAYER_UPDATE))
                .route(web::put().to(crate::http::handlers::ins_payers::update::update_ins_payer))
        )
        .service(
            web::resource("/api/v1/ins/payers/{id}")
                .wrap(RequirePermission::new(perm::PAYER_DELETE))
                .route(web::delete().to(crate::http::handlers::ins_payers::delete::delete_ins_payer))
        )
        // Insurance Policies
        .service(
            web::resource("/api/v1/ins/policies")
                .wrap(RequirePermission::new(perm::POLICY_CREATE))
                .route(web::post().to(crate::http::handlers::ins_policies::create::create_ins_policy))
        )
        .service(
            web::resource("/api/v1/ins/policies")
                .wrap(RequirePermission::new(perm::POLICY_GET))
                .route(web::get().to(crate::http::handlers::ins_policies::list::list_ins_policies))
        )
        .service(
            web::resource("/api/v1/ins/policies/{id}")
                .wrap(RequirePermission::new(perm::POLICY_GET))
                .route(web::get().to(crate::http::handlers::ins_policies::get::get_ins_policy))
        )
        .service(
            web::resource("/api/v1/ins/policies/{id}")
                .wrap(RequirePermission::new(perm::POLICY_UPDATE))
                .route(web::put().to(crate::http::handlers::ins_policies::update::update_ins_policy))
        )
        .service(
            web::resource("/api/v1/ins/policies/{id}")
                .wrap(RequirePermission::new(perm::POLICY_DELETE))
                .route(web::delete().to(crate::http::handlers::ins_policies::delete::delete_ins_policy))
        )
}
