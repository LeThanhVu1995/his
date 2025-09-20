use actix_web::{web, Scope};
use crate::http::handlers::{
    health,
    medications::{list, create, update},
    prescriptions::{list as prescription_list, create as prescription_create, update as prescription_update},
    dispenses::{list as dispense_list, create as dispense_create, finish as dispense_finish},
};

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(health::healthz))
        // Medications
        .service(
            web::resource("/api/v1/medications")
                .route(web::get().to(list::list_meds)),
        )
        .service(
            web::resource("/api/v1/medications:create")
                .route(web::post().to(create::create_med)),
        )
        .service(
            web::resource("/api/v1/medications/{id}")
                .route(web::put().to(update::update_med)),
        )
        // Prescriptions
        .service(
            web::resource("/api/v1/prescriptions")
                .route(web::get().to(prescription_list::list_prescriptions)),
        )
        .service(
            web::resource("/api/v1/prescriptions:create")
                .route(web::post().to(prescription_create::create_prescription)),
        )
        .service(
            web::resource("/api/v1/prescriptions/{id}")
                .route(web::put().to(prescription_update::update_prescription)),
        )
        // Dispenses
        .service(
            web::resource("/api/v1/dispenses")
                .route(web::get().to(dispense_list::list_dispenses)),
        )
        .service(
            web::resource("/api/v1/dispenses:create")
                .route(web::post().to(dispense_create::create_dispense)),
        )
        .service(
            web::resource("/api/v1/dispenses/{id}:finish")
                .route(web::put().to(dispense_finish::finish_dispense)),
        )
}
