use actix_web::{web, Scope};

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(crate::http::handlers::health::healthz))
        // Procedures - simplified without permission middleware
        .route("/api/v1/ris/procedures", web::get().to(crate::http::handlers::procedures::list_procedures))
        .route("/api/v1/ris/procedures:create", web::post().to(crate::http::handlers::procedures::create_procedure))
        .route("/api/v1/ris/procedures/{id}", web::put().to(crate::http::handlers::procedures::update_procedure))
        // Orders - simplified without permission middleware
        .route("/api/v1/ris/orders", web::get().to(crate::http::handlers::orders::list_orders))
        .route("/api/v1/ris/orders:create", web::post().to(crate::http::handlers::orders::create_order))
        .route("/api/v1/ris/orders/{id}", web::put().to(crate::http::handlers::orders::update_order))
        // Studies - simplified without permission middleware
        .route("/api/v1/ris/studies", web::get().to(crate::http::handlers::studies::list_studies))
        .route("/api/v1/ris/studies:create", web::post().to(crate::http::handlers::studies::create_study))
        .route("/api/v1/ris/studies/{id}:progress", web::put().to(crate::http::handlers::studies::progress_study))
        // Reports - simplified without permission middleware
        .route("/api/v1/ris/reports", web::get().to(crate::http::handlers::reports::list_reports))
        .route("/api/v1/ris/reports:create", web::post().to(crate::http::handlers::reports::create_report))
        .route("/api/v1/ris/reports/{id}:edit", web::put().to(crate::http::handlers::reports::edit_report))
        .route("/api/v1/ris/reports/{id}:verify", web::put().to(crate::http::handlers::reports::verify_report))
        .route("/api/v1/ris/reports/{id}:final", web::put().to(crate::http::handlers::reports::final_report))
}
