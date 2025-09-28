use actix_web::web;
use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

pub mod handlers;
pub mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::handlers::health::healthz,
    ),
    components(schemas(
        crate::dto::PaginationQuery,
    )),
    modifiers(&SecurityAddon)
)]
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
        .service(
            utoipa_swagger_ui::SwaggerUi::new("/swagger")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .route("/api-docs/openapi.json", web::get().to(|| async {
            actix_web::HttpResponse::Ok().json(ApiDoc::openapi())
        }))
        .service(crate::http::routes::api_scope());
}
