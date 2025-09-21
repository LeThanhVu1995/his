use actix_web::{web, Scope};
use crate::http::handlers;

pub fn api_scope() -> Scope {
    web::scope("")
        .service(
            web::resource("/healthz")
                .route(web::get().to(handlers::health::healthz))
        )
        // Medications
        .service(
            web::resource("/api/v1/medications")
                .route(web::get().to(handlers::medications::list_meds))
        )
        .service(
            web::resource("/api/v1/medications:create")
                .route(web::post().to(handlers::medications::create_med))
        )
        .service(
            web::resource("/api/v1/medications/{id}")
                .route(web::put().to(handlers::medications::update_med))
        )
        // Prescriptions
        .service(
            web::resource("/api/v1/prescriptions")
                .route(web::get().to(handlers::prescriptions::list_prescriptions))
        )
        .service(
            web::resource("/api/v1/prescriptions:create")
                .route(web::post().to(handlers::prescriptions::create_prescription))
        )
        .service(
            web::resource("/api/v1/prescriptions/{id}")
                .route(web::put().to(handlers::prescriptions::update_prescription))
        )
        // Dispenses
        .service(
            web::resource("/api/v1/dispenses")
                .route(web::get().to(handlers::dispenses::list_dispenses))
        )
        .service(
            web::resource("/api/v1/dispenses:create")
                .route(web::post().to(handlers::dispenses::create_dispense))
        )
        .service(
            web::resource("/api/v1/dispenses/{id}:finish")
                .route(web::put().to(handlers::dispenses::finish_dispense))
        )
}
