use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;

pub fn api_scope()->Scope{
  web::scope("")
    .service(crate::http::handlers::health::healthz)
    // OR
    .service(web::resource("/api/v1/or/schedules")
      .wrap(RequirePermission::new(crate::security::policy::perm::OR_SCHEDULE_CREATE))
      .route(web::post().to(crate::http::handlers::schedules::create::create)))
    .service(web::resource("/api/v1/or/schedules:by-room")
      .wrap(RequirePermission::new(crate::security::policy::perm::OR_SCHEDULE_VIEW))
      .route(web::get().to(crate::http::handlers::schedules::list::list_by_room)))
    .service(web::resource("/api/v1/or/procedures:start")
      .wrap(RequirePermission::new(crate::security::policy::perm::OR_PROCEDURE_START))
      .route(web::post().to(crate::http::handlers::procedures::start::start)))
    .service(web::resource("/api/v1/or/procedures:complete")
      .wrap(RequirePermission::new(crate::security::policy::perm::OR_PROCEDURE_COMPLETE))
      .route(web::post().to(crate::http::handlers::procedures::complete::complete)))
    // CSSD
    .service(web::resource("/api/v1/cssd/trays:sterilize")
      .wrap(RequirePermission::new(crate::security::policy::perm::CSSD_STERILIZE))
      .route(web::post().to(crate::http::handlers::cssd::sterilize::sterilize)))
    .service(web::resource("/api/v1/cssd/trays:issue")
      .wrap(RequirePermission::new(crate::security::policy::perm::CSSD_ISSUE))
      .route(web::post().to(crate::http::handlers::cssd::issue::issue)))
}
