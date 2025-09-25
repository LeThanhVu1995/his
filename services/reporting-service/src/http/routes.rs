use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;
use crate::security::policy::perm;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(crate::http::handlers::health::healthz))
        .service(
            web::resource("/api/v1/rpt/dashboards/overview")
                .wrap(RequirePermission::new(perm::DASHBOARD_VIEW))
                .route(web::get().to(crate::http::handlers::dashboards::overview))
        )
        .service(
            web::resource("/api/v1/rpt/queries:adhoc")
                .wrap(RequirePermission::new(perm::QUERY_RUN))
                .route(web::post().to(crate::http::handlers::queries::adhoc))
        )
        .service(
            web::resource("/api/v1/rpt/exports/revenue.xlsx")
                .wrap(RequirePermission::new(perm::EXPORT))
                .route(web::get().to(crate::http::handlers::exports::revenue_xlsx))
        )
}
