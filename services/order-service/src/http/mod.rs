use actix_web::{web, HttpResponse};
use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

use crate::dto::order_dto::{
    CreateOrderReq, UpdateOrderReq, OrderQuery, OrderRes, UpdateItemReq, SubmitResultReq, OrderItemRes,
};

pub mod routes;
pub mod handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::handlers::orders::list_orders,
        crate::http::handlers::orders::create_order,
        crate::http::handlers::orders::update_order,
        crate::http::handlers::items::list_items,
        crate::http::handlers::items::update_item,
        crate::http::handlers::items::submit_result,
    ),
    components(schemas(CreateOrderReq, UpdateOrderReq, OrderQuery, OrderRes, UpdateItemReq, SubmitResultReq, OrderItemRes)),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components = Some(utoipa::openapi::ComponentsBuilder::new()
            .security_scheme("bearer_auth", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)))
            .build());
    }
}

pub fn mount(cfg: &mut web::ServiceConfig) {
    cfg.service(
        utoipa_swagger_ui::SwaggerUi::new("/swagger")
            .url("/api-docs/openapi.json", ApiDoc::openapi())
    )
    .route(
        "/api-docs/openapi.json",
        web::get().to(|| async { HttpResponse::Ok().json(ApiDoc::openapi()) })
    )
    .service(crate::http::routes::api_scope());
}
