use actix_web::{web, HttpResponse};
use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

use crate::dto::code_dto::{CreateCodeReq, UpdateCodeReq, CodeRes, BulkCreateCodeReq, BulkUpdateCodeReq, BulkCreateCodeRes, BulkError};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::handlers::codes::list_codes,
        crate::http::handlers::codes::create_code,
        crate::http::handlers::codes::update_code,
        crate::http::handlers::codes::delete_code,
        crate::http::handlers::codes::bulk_create_codes,
        crate::http::handlers::codes::bulk_update_codes,
    ),
    components(schemas(CreateCodeReq, UpdateCodeReq, CodeRes, BulkCreateCodeReq, BulkUpdateCodeReq, BulkCreateCodeRes, BulkError)),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components = Some(
            utoipa::openapi::ComponentsBuilder::new()
                .security_scheme(
                    "bearer_auth",
                    SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer))
                )
                .build()
        );
    }
}

pub fn mount(cfg: &mut web::ServiceConfig) {
    cfg.service(
        utoipa_swagger_ui::SwaggerUi::new("/swagger")
            .url("/api-docs/openapi.json", ApiDoc::openapi())
    )
    .route("/api-docs/openapi.json", web::get().to(|| async {
        HttpResponse::Ok().json(ApiDoc::openapi())
    }))
    .service(crate::http::routes::api_scope());
}

pub mod handlers;
pub mod routes;
pub mod middleware;

