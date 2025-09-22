use actix_web::{web, HttpResponse};
use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

use crate::dto::warehouse_dto::{CreateWarehouseReq, UpdateWarehouseReq, WarehouseQuery, WarehouseRes};
use crate::dto::item_dto::{CreateItemReq, UpdateItemReq, ItemQuery, ItemRes};
use crate::dto::lot_dto::{CreateLotReq, LotQuery, LotRes};
use crate::dto::stock_dto::StockRes;
use crate::dto::movement_dto::{ReceiveReq, IssueReq, TransferReq, AdjustReq, MovementQuery, MovementRes};

pub mod handlers;
pub mod routes;

// #[derive(OpenApi)]
// #[openapi(
//     paths(
//         crate::http::handlers::warehouses::list_warehouses,
//         crate::http::handlers::warehouses::create_warehouse,
//         crate::http::handlers::warehouses::update_warehouse,
//         crate::http::handlers::items::list_items,
//         crate::http::handlers::items::create_item,
//         crate::http::handlers::items::update_item,
//         crate::http::handlers::lots::list_lots,
//         crate::http::handlers::lots::create_lot,
//         crate::http::handlers::stocks::list_stocks,
//         crate::http::handlers::movements::list_movements,
//         crate::http::handlers::movements::receive_stocks,
//         crate::http::handlers::movements::issue_stocks,
//         crate::http::handlers::movements::transfer_stocks,
//         crate::http::handlers::movements::adjust_stocks,
//     ),
//     components(schemas(
//         CreateWarehouseReq, UpdateWarehouseReq, WarehouseQuery, WarehouseRes,
//         CreateItemReq, UpdateItemReq, ItemQuery, ItemRes,
//         CreateLotReq, LotQuery, LotRes,
//         StockRes,
//         ReceiveReq, IssueReq, TransferReq, AdjustReq, MovementQuery, MovementRes
//     )),
//     modifiers(&SecurityAddon)
// )]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components = Some(
            utoipa::openapi::ComponentsBuilder::new()
                .security_scheme("bearer_auth", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)))
                .build()
        );
    }
}

pub fn mount(cfg: &mut web::ServiceConfig) {
    cfg
    // .service(
    //     utoipa_swagger_ui::SwaggerUi::new("/swagger")
    //         .url("/api-docs/openapi.json", ApiDoc::openapi())
    // )
    // .route("/api-docs/openapi.json", web::get().to(|| async { HttpResponse::Ok().json(ApiDoc::openapi()) }))
    .service(crate::http::routes::api_scope());
}
