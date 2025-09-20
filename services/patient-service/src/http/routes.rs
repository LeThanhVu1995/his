use actix_web::{web, Scope};
use crate::http::handlers;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(handlers::health::healthz))
        // Patients
        .service(
            web::resource("/api/v1/patients")
                .route(web::get().to(handlers::patients::list_patients))
        )
        .service(
            web::resource("/api/v1/patients:create")
                .route(web::post().to(handlers::patients::create_patient))
        )
        .service(
            web::resource("/api/v1/patients/{id}")
                .route(web::get().to(handlers::patients::get_patient))
                .route(web::put().to(handlers::patients::update_patient))
        )
        // Encounters
        .service(
            web::resource("/api/v1/encounters")
                .route(web::get().to(handlers::encounters::list_encounters))
        )
        .service(
            web::resource("/api/v1/encounters:create")
                .route(web::post().to(handlers::encounters::create_encounter))
        )
        .service(
            web::resource("/api/v1/encounters/{id}")
                .route(web::put().to(handlers::encounters::update_encounter))
        )
        .service(
            web::resource("/api/v1/encounters/{id}:close")
                .route(web::put().to(handlers::encounters::close_encounter))
        )
}
