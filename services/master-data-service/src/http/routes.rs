use actix_web::{web, Scope};
use crate::http::handlers;
use crate::security::permission::RequirePermission;
use crate::security::policy::perm;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(handlers::health::healthz))
        // List: cần his.master.code.list
        .service(
            web::resource("/api/v1/master/codes")
                .wrap(RequirePermission::new(perm::MASTER_CODE_LIST))
                .route(web::get().to(handlers::codes::list_codes))
        )
        // Create: cần his.master.code.create
        .service(
            web::resource("/api/v1/master/codes")
                .wrap(RequirePermission::new(perm::MASTER_CODE_CREATE))
                .route(web::post().to(handlers::codes::create_code))
        )
        // Bulk operations
        .service(
            web::resource("/api/v1/master/codes/bulk")
                .wrap(RequirePermission::new(perm::MASTER_CODE_CREATE))
                .route(web::post().to(handlers::codes::bulk_create_codes))
                .route(web::put().to(handlers::codes::bulk_update_codes))
        )
        // Detail/Update/Delete
        .service(
            web::resource("/api/v1/master/codes/{id}")
                .wrap(RequirePermission::new(perm::MASTER_CODE_READ))
                .route(web::put().to(handlers::codes::update_code))
                .route(web::delete().to(handlers::codes::delete_code))
        )
}
