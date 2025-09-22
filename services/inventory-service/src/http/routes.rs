use actix_web::{web, Scope};
use crate::http::handlers;
use crate::security::permission::RequirePermission;
use crate::security::policy::perm;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(handlers::health::healthz))
        // Warehouses
        .service(
            web::resource("/api/v1/inv/warehouses")
                .wrap(RequirePermission::new(perm::WH_LIST))
                .route(web::get().to(handlers::warehouses::list_warehouses))
        )
        .service(
            web::resource("/api/v1/inv/warehouses:create")
                .wrap(RequirePermission::new(perm::WH_CREATE))
                .route(web::post().to(handlers::warehouses::create_warehouse))
        )
        .service(
            web::resource("/api/v1/inv/warehouses/{id}")
                .wrap(RequirePermission::new(perm::WH_UPDATE))
                .route(web::put().to(handlers::warehouses::update_warehouse))
        )
        // Items
        .service(
            web::resource("/api/v1/inv/items")
                .wrap(RequirePermission::new(perm::ITEM_LIST))
                .route(web::get().to(handlers::items::list_items))
        )
        .service(
            web::resource("/api/v1/inv/items:create")
                .wrap(RequirePermission::new(perm::ITEM_CREATE))
                .route(web::post().to(handlers::items::create_item))
        )
        .service(
            web::resource("/api/v1/inv/items/{id}")
                .wrap(RequirePermission::new(perm::ITEM_UPDATE))
                .route(web::put().to(handlers::items::update_item))
        )
        // Lots
        .service(
            web::resource("/api/v1/inv/lots")
                .wrap(RequirePermission::new(perm::LOT_LIST))
                .route(web::get().to(handlers::lots::list_lots))
        )
        .service(
            web::resource("/api/v1/inv/lots:create")
                .wrap(RequirePermission::new(perm::LOT_CREATE))
                .route(web::post().to(handlers::lots::create_lot))
        )
        // Stocks
        .service(
            web::resource("/api/v1/inv/stocks")
                .wrap(RequirePermission::new(perm::STOCK_VIEW))
                .route(web::get().to(handlers::stocks::list_stocks))
        )
        // Movements
        .service(
            web::resource("/api/v1/inv/movements")
                .wrap(RequirePermission::new(perm::MOVE_LIST))
                .route(web::get().to(handlers::movements::list_movements))
        )
        .service(
            web::resource("/api/v1/inv/movements:receive")
                .wrap(RequirePermission::new(perm::MOVE_RECEIVE))
                .route(web::post().to(handlers::movements::receive_stocks))
        )
        .service(
            web::resource("/api/v1/inv/movements:issue")
                .wrap(RequirePermission::new(perm::MOVE_ISSUE))
                .route(web::post().to(handlers::movements::issue_stocks))
        )
        .service(
            web::resource("/api/v1/inv/movements:transfer")
                .wrap(RequirePermission::new(perm::MOVE_TRANSFER))
                .route(web::post().to(handlers::movements::transfer_stocks))
        )
        .service(
            web::resource("/api/v1/inv/movements:adjust")
                .wrap(RequirePermission::new(perm::MOVE_ADJUST))
                .route(web::post().to(handlers::movements::adjust_stocks))
        )
}
