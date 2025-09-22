use actix_web::{web, HttpResponse};
use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

pub mod handlers;
pub mod routes;

use crate::dto::procedure_dto::{CreateProcedureReq, UpdateProcedureReq, ProcQuery, ProcedureRes};
use crate::dto::order_dto::{CreateOrderReq, UpdateOrderReq, OrderQuery, OrderRes};
use crate::dto::study_dto::{CreateStudyReq, ProgressStudyReq, StudyQuery, StudyRes};
use crate::dto::report_dto::{CreateReportReq, EditReportReq, VerifyReportReq, FinalizeReportReq, ReportQuery, ReportRes};

// OpenAPI documentation temporarily disabled
// #[derive(OpenApi)]
// #[openapi(
//     paths(
//         crate::http::handlers::procedures::list_procedures,
//         crate::http::handlers::procedures::create_procedure,
//         crate::http::handlers::procedures::update_procedure,
//         crate::http::handlers::orders::list_orders,
//         crate::http::handlers::orders::create_order,
//         crate::http::handlers::orders::update_order,
//         crate::http::handlers::studies::list_studies,
//         crate::http::handlers::studies::create_study,
//         crate::http::handlers::studies::progress_study,
//         crate::http::handlers::reports::list_reports,
//         crate::http::handlers::reports::create_report,
//         crate::http::handlers::reports::edit_report,
//         crate::http::handlers::reports::verify_report,
//         crate::http::handlers::reports::final_report,
//     ),
//     components(schemas(
//         CreateProcedureReq, UpdateProcedureReq, ProcQuery, ProcedureRes,
//         CreateOrderReq, UpdateOrderReq, OrderQuery, OrderRes,
//         CreateStudyReq, ProgressStudyReq, StudyQuery, StudyRes,
//         CreateReportReq, EditReportReq, VerifyReportReq, FinalizeReportReq, ReportQuery, ReportRes
//     )),
//     modifiers(&SecurityAddon)
// )]
// pub struct ApiDoc;

// pub struct SecurityAddon;
// impl Modify for SecurityAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         openapi.components = Some(utoipa::openapi::ComponentsBuilder::new()
//             .security_scheme("bearer_auth", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)))
//             .build());
//     }
// }

pub fn mount(cfg: &mut web::ServiceConfig) {
    cfg
        // Swagger UI temporarily disabled
        // .service(utoipa_swagger_ui::SwaggerUi::new("/swagger")
        //     .url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .route("/api-docs/openapi.json", web::get().to(|| async { HttpResponse::Ok().json(ApiDoc::openapi()) }))
        .service(crate::http::routes::api_scope());
}
