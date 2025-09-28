use actix_web::{web, Scope};
use crate::http::handlers;

pub fn api_scope() -> Scope {
    web::scope("")
        .service(
            web::resource("/healthz")
                .route(web::get().to(handlers::health::healthz))
        )
        // Drug Catalog - Real handlers
        .service(
            web::resource("/api/v1/pharmacy/drug-catalog")
                .route(web::get().to(handlers::drug_catalog::list_drug_catalog))
                .route(web::post().to(handlers::drug_catalog::create_drug_catalog))
        )
        .service(
            web::resource("/api/v1/pharmacy/drug-catalog/{id}")
                .route(web::get().to(handlers::drug_catalog::get_drug_catalog))
                .route(web::put().to(handlers::drug_catalog::update_drug_catalog))
                .route(web::delete().to(handlers::drug_catalog::delete_drug_catalog))
        )
        .service(
            web::resource("/api/v1/pharmacy/drug-catalog/stats")
                .route(web::get().to(handlers::drug_catalog::get_drug_catalog_stats))
        )
        // Prescriptions - Real handlers
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
        // Dispenses - Real handlers
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
