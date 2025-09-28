use actix_web::{web, Scope};
use crate::http::handlers;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(handlers::health::healthz))
        // List
        .service(
            web::resource("/api/v1/master/codes")
                .route(web::get().to(handlers::codes::list_codes))
        )
        // Create
        .service(
            web::resource("/api/v1/master/codes")
                .route(web::post().to(handlers::codes::create_code))
        )
        // Code Sets
        .service(
            web::resource("/api/v1/master/code-sets")
                .route(web::get().to(handlers::sets::list_sets))
                .route(web::post().to(handlers::sets::create_set))
        )
        .service(
            web::resource("/api/v1/master/code-sets/{set_code}/codes")
                .route(web::get().to(handlers::sets::list_codes_in_set))
                .route(web::post().to(handlers::sets::create_code_in_set))
        )
        // Bulk operations
        .service(
            web::resource("/api/v1/master/codes/bulk")
                .route(web::post().to(handlers::codes::bulk_create_codes))
                .route(web::put().to(handlers::codes::bulk_update_codes))
        )
        // Detail/Update/Delete
        .service(
            web::resource("/api/v1/master/codes/{id}")
                .route(web::put().to(handlers::codes::update_code))
                .route(web::delete().to(handlers::codes::delete_code))
        )
        // Organization Management
        .service(
            web::resource("/api/v1/master/hospitals")
                .route(web::get().to(handlers::organization::list_hospitals))
                .route(web::post().to(handlers::organization::create_hospital))
        )
        .service(
            web::resource("/api/v1/master/hospitals/{id}")
                .route(web::get().to(handlers::organization::get_hospital))
                .route(web::put().to(handlers::organization::update_hospital))
        )
        .service(
            web::resource("/api/v1/master/facilities")
                .route(web::get().to(handlers::organization::list_facilities))
                .route(web::post().to(handlers::organization::create_facility))
        )
        // Inventory Management
        .service(
            web::resource("/api/v1/master/uoms")
                .route(web::get().to(handlers::inventory::list_uoms))
                .route(web::post().to(handlers::inventory::create_uom))
        )
        .service(
            web::resource("/api/v1/master/uoms/{id}")
                .route(web::get().to(handlers::inventory::get_uom))
                .route(web::put().to(handlers::inventory::update_uom))
                .route(web::delete().to(handlers::inventory::delete_uom))
        )
}
