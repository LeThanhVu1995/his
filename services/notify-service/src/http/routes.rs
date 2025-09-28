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
        // Notifications (aligned with root.sql)
        .service(
            web::resource("/api/v1/notify/notifications")
                .wrap(RequirePermission::new(perm::NOTIFICATION_LIST))
                .route(web::get().to(crate::http::handlers::notifications::list_notifications))
                .route(web::post().to(crate::http::handlers::notifications::create_notification))
        )
        .service(
            web::resource("/api/v1/notify/notifications/{id}")
                .wrap(RequirePermission::new(perm::NOTIFICATION_GET))
                .route(web::get().to(crate::http::handlers::notifications::get_notification))
                .route(web::put().to(crate::http::handlers::notifications::update_notification))
                .route(web::delete().to(crate::http::handlers::notifications::delete_notification))
        )
        .service(
            web::resource("/api/v1/notify/notifications/{id}/targets")
                .wrap(RequirePermission::new(perm::NOTIFICATION_ASSIGN))
                .route(web::post().to(crate::http::handlers::notifications::assign_notification))
        )
        .service(
            web::resource("/api/v1/notify/notifications/{id}/targets/bulk")
                .wrap(RequirePermission::new(perm::NOTIFICATION_ASSIGN))
                .route(web::post().to(crate::http::handlers::notifications::assign_notification_bulk))
        )
        .service(
            web::resource("/api/v1/notify/users/{user_id}/notifications")
                .wrap(RequirePermission::new(perm::NOTIFICATION_READ))
                .route(web::get().to(crate::http::handlers::notifications::get_user_notifications))
        )
        .service(
            web::resource("/api/v1/notify/users/{user_id}/notifications/stats")
                .wrap(RequirePermission::new(perm::NOTIFICATION_READ))
                .route(web::get().to(crate::http::handlers::notifications::get_user_notification_stats))
        )
        .service(
            web::resource("/api/v1/notify/users/{user_id}/notifications/{notification_id}/read")
                .wrap(RequirePermission::new(perm::NOTIFICATION_READ))
                .route(web::put().to(crate::http::handlers::notifications::mark_notification_read))
        )
}
