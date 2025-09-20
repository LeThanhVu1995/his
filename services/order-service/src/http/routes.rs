use actix_web::{web, Scope};
use crate::http::handlers;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(handlers::health::healthz))
        // Orders
        .service(
            web::resource("/api/v1/orders")
                .route(web::get().to(handlers::orders::list_orders))
        )
        .service(
            web::resource("/api/v1/orders:create")
                .route(web::post().to(handlers::orders::create_order))
        )
        .service(
            web::resource("/api/v1/orders/{id}")
                .route(web::put().to(handlers::orders::update_order))
        )
        // Items
        .service(
            web::resource("/api/v1/orders/{order_id}/items")
                .route(web::get().to(handlers::items::list_items))
        )
        .service(
            web::resource("/api/v1/order-items/{id}")
                .route(web::put().to(handlers::items::update_item))
        )
        .service(
            web::resource("/api/v1/order-items/{id}:result")
                .route(web::post().to(handlers::items::submit_result))
        )
}
