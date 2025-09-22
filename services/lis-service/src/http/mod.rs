use actix_web::{web, HttpResponse};
use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use crate::http::dto::test_dto::{CreateTestReq, UpdateTestReq, TestQuery, LabTestRes};
use crate::http::dto::specimen_dto::{CreateSpecimenReq, SpecimenQuery, SpecimenRes};
use crate::http::dto::result_dto::{CreateResultReq, EnterResultReq, ResultQuery, LabResultRes};

pub mod dto;
pub mod handlers;
pub mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::handlers::tests::list_tests,
        crate::http::handlers::tests::create_test,
        crate::http::handlers::tests::update_test,
        crate::http::handlers::specimens::list_specimens,
        crate::http::handlers::specimens::create_specimen,
        crate::http::handlers::specimens::collect_specimen,
        crate::http::handlers::specimens::receive_specimen,
        crate::http::handlers::specimens::reject_specimen,
        crate::http::handlers::results::list_results,
        crate::http::handlers::results::create_result,
        crate::http::handlers::results::enter_values,
        crate::http::handlers::results::verify_result,
        crate::http::handlers::results::release_result,
    ),
    components(schemas(
        CreateTestReq, UpdateTestReq, TestQuery, LabTestRes,
        CreateSpecimenReq, SpecimenQuery, SpecimenRes,
        CreateResultReq, EnterResultReq, ResultQuery, LabResultRes
    )),
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
    cfg.service(utoipa_swagger_ui::SwaggerUi::new("/swagger")
        .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api-docs/openapi.json", web::get().to(|| async { HttpResponse::Ok().json(ApiDoc::openapi()) }))
        .service(crate::http::routes::api_scope());
}
