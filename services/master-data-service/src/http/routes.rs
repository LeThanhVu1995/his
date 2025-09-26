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
}
