use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;
use crate::security::policy::perm;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(crate::http::handlers::health::healthz))
        // Templates
        .service(
            web::resource("/api/v1/notify/templates")
                .wrap(RequirePermission::new(perm::TEMPLATE_LIST))
                .route(web::get().to(crate::http::handlers::templates::list::list))
        )
        .service(
            web::resource("/api/v1/notify/templates:create")
                .wrap(RequirePermission::new(perm::TEMPLATE_CREATE))
                .route(web::post().to(crate::http::handlers::templates::create::create))
        )
        .service(
            web::resource("/api/v1/notify/templates:render")
                .wrap(RequirePermission::new(perm::TEMPLATE_RENDER))
                .route(web::post().to(crate::http::handlers::templates::render::render))
        )
        // Messages
        .service(
            web::resource("/api/v1/notify/messages:send")
                .wrap(RequirePermission::new(perm::MESSAGE_SEND))
                .route(web::post().to(crate::http::handlers::messages::send::send))
        )
        .service(
            web::resource("/api/v1/notify/messages/{id}")
                .wrap(RequirePermission::new(perm::MESSAGE_GET))
                .route(web::get().to(crate::http::handlers::messages::get::get))
        )
        // Webhooks
        .service(
            web::resource("/api/v1/notify/webhooks:register")
                .wrap(RequirePermission::new(perm::WEBHOOK_REGISTER))
                .route(web::post().to(crate::http::handlers::webhooks::register::register))
        )
        .service(
            web::resource("/api/v1/notify/webhooks:trigger")
                .wrap(RequirePermission::new(perm::WEBHOOK_TRIGGER))
                .route(web::post().to(crate::http::handlers::webhooks::trigger::trigger))
        )
}
