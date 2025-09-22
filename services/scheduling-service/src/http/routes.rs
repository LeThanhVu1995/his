use actix_web::{web, Scope};
use crate::http::handlers;
use crate::security::permission::RequirePermission;
use crate::security::policy::perm;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(handlers::health::healthz))
        // Providers
        .route("/api/v1/appt/providers", web::get().to(handlers::providers::list_providers))
        .route("/api/v1/appt/providers:create", web::post().to(handlers::providers::create_provider))
        .route("/api/v1/appt/providers/{id}", web::put().to(handlers::providers::update_provider))
        // Rooms
        .route("/api/v1/appt/rooms", web::get().to(handlers::rooms::list_rooms))
        .route("/api/v1/appt/rooms:create", web::post().to(handlers::rooms::create_room))
        .route("/api/v1/appt/rooms/{id}", web::put().to(handlers::rooms::update_room))
        // Schedules
        .route("/api/v1/appt/schedules", web::get().to(handlers::schedules::list_schedules))
        .route("/api/v1/appt/schedules:create", web::post().to(handlers::schedules::create_schedule))
        .route("/api/v1/appt/schedules/{id}", web::put().to(handlers::schedules::update_schedule))
        // Slots
        .route("/api/v1/appt/slots", web::get().to(handlers::slots::list_slots))
        .route("/api/v1/appt/slots:generate", web::post().to(handlers::slots::generate_slots))
        // Appointments
        .route("/api/v1/appt/appointments", web::get().to(handlers::appointments::list_appts))
        .route("/api/v1/appt/appointments:book", web::post().to(handlers::appointments::book_appt))
        .route("/api/v1/appt/appointments/{id}:cancel", web::put().to(handlers::appointments::cancel_appt))
        .route("/api/v1/appt/appointments/{id}:reschedule", web::put().to(handlers::appointments::reschedule_appt))
}
