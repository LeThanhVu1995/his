use actix_web::{web, HttpResponse};
use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

use crate::dto::patient_dto::{CreatePatientReq, UpdatePatientReq, PatientRes};
use crate::dto::encounter_dto::{CreateEncounterReq, UpdateEncounterReq, EncounterRes};

pub mod handlers;
pub mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::handlers::patients::list_patients,
        crate::http::handlers::patients::create_patient,
        crate::http::handlers::patients::get_patient,
        crate::http::handlers::patients::update_patient,
        crate::http::handlers::encounters::list_encounters,
        crate::http::handlers::encounters::create_encounter,
        crate::http::handlers::encounters::update_encounter,
        crate::http::handlers::encounters::close_encounter,
    ),
    components(schemas(CreatePatientReq, UpdatePatientReq, PatientRes, CreateEncounterReq, UpdateEncounterReq, EncounterRes)),
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
    cfg.service(utoipa_swagger_ui::SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api-docs/openapi.json", web::get().to(|| async { HttpResponse::Ok().json(ApiDoc::openapi()) }))
        .service(crate::http::routes::api_scope());
}
