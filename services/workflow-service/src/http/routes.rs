use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;
use crate::security::policy::perm;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(crate::http::handlers::health::healthz))
        // Templates
        .service(
            web::resource("/api/v1/wf/templates:upsert")
                .wrap(RequirePermission::new(perm::TEMPLATE_UPSERT))
                .route(web::post().to(crate::api::templates::upsert))
        )
        .service(
            web::resource("/api/v1/wf/templates/{code}")
                .wrap(RequirePermission::new(perm::TEMPLATE_GET))
                .route(web::get().to(crate::api::templates::get))
        )
        .service(
            web::resource("/api/v1/wf/templates")
                .wrap(RequirePermission::new(perm::TEMPLATE_GET))
                .route(web::get().to(crate::api::templates::list))
        )
        // Instances
        .service(
            web::resource("/api/v1/wf/instances:start/{code}")
                .wrap(RequirePermission::new(perm::INSTANCE_START))
                .route(web::post().to(crate::api::instances::start))
        )
        .service(
            web::resource("/api/v1/wf/instances/{id}")
                .wrap(RequirePermission::new(perm::INSTANCE_GET))
                .route(web::get().to(crate::api::instances::get))
        )
        // Tasks
        .service(
            web::resource("/api/v1/wf/tasks/{id}:claim")
                .wrap(RequirePermission::new(perm::TASK_CLAIM))
                .route(web::post().to(crate::api::tasks::claim))
        )
        .service(
            web::resource("/api/v1/wf/tasks/{id}:complete")
                .wrap(RequirePermission::new(perm::TASK_COMPLETE))
                .route(web::post().to(crate::api::tasks::complete))
        )
        .service(
            web::resource("/api/v1/wf/tasks/{id}")
                .wrap(RequirePermission::new(perm::TASK_CLAIM))
                .route(web::get().to(crate::api::tasks::get))
        )
        // Observability
        .service(
            web::resource("/api/v1/wf/observability/health")
                .wrap(RequirePermission::new(perm::OBSERVABILITY_HEALTH))
                .route(web::get().to(crate::api::observability::health))
        )
        .service(
            web::resource("/api/v1/wf/observability/metrics")
                .wrap(RequirePermission::new(perm::OBSERVABILITY_HEALTH))
                .route(web::get().to(crate::api::observability::metrics))
        )
}
