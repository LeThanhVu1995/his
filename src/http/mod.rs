use actix_web::{web, HttpResponse};
use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

use crate::http::dto::medication_dto::{CreateMedicationReq, UpdateMedicationReq, MedQuery, MedicationRes};
use crate::http::dto::prescription_dto::{CreatePrescriptionReq, UpdatePrescriptionReq, PrescQuery, PrescriptionRes};
use crate::http::dto::dispense_dto::{CreateDispenseReq, DispenseQuery, DispenseRes};

pub mod routes;
pub mod handlers;
pub mod dto;

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        CreateMedicationReq, UpdateMedicationReq, MedQuery, MedicationRes,
        CreatePrescriptionReq, UpdatePrescriptionReq, PrescQuery, PrescriptionRes,
        CreateDispenseReq, DispenseQuery, DispenseRes
    )),
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
                    SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
                )
                .build(),
        );
    }
}

pub fn mount(cfg: &mut web::ServiceConfig) {
    cfg.service(
        utoipa_swagger_ui::SwaggerUi::new("/swagger")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
    .route(
        "/api-docs/openapi.json",
        web::get().to(|| async { HttpResponse::Ok().json(ApiDoc::openapi()) }),
    )
    .service(crate::http::routes::api_scope());
}
