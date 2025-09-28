use actix_web::{web, Scope};
use crate::http::handlers::{
    orders::{create::create_imaging_order, get::{list_imaging_orders, get_imaging_order}},
    studies::{schedule::schedule_study, start::start_study, complete::complete_study},
    reports::{get::{list_reports, get_report}, finalize::{create_report, update_report, finalize_report}},
    health::healthz,
};

pub fn api_scope() -> Scope {
    web::scope("")
        .service(
            web::resource("/healthz")
                .route(web::get().to(healthz))
        )
        // Imaging Orders
        .service(
            web::resource("/api/v1/ris-pacs/orders")
                .route(web::get().to(list_imaging_orders))
                .route(web::post().to(create_imaging_order))
        )
        .service(
            web::resource("/api/v1/ris-pacs/orders/{id}")
                .route(web::get().to(get_imaging_order))
        )
        // Studies
        .service(
            web::resource("/api/v1/ris-pacs/studies")
                .route(web::post().to(schedule_study))
        )
        .service(
            web::resource("/api/v1/ris-pacs/studies/{id}/start")
                .route(web::put().to(start_study))
        )
        .service(
            web::resource("/api/v1/ris-pacs/studies/{id}/complete")
                .route(web::put().to(complete_study))
        )
        // Reports
        .service(
            web::resource("/api/v1/ris-pacs/reports")
                .route(web::get().to(list_reports))
                .route(web::post().to(create_report))
        )
        .service(
            web::resource("/api/v1/ris-pacs/reports/{id}")
                .route(web::get().to(get_report))
                .route(web::put().to(update_report))
        )
        .service(
            web::resource("/api/v1/ris-pacs/reports/{id}/finalize")
                .route(web::put().to(finalize_report))
        )
}