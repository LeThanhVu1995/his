use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;

pub fn api_scope()->Scope{
  web::scope("")
    .service(crate::http::handlers::health::healthz)
    .service(web::resource("/api/v1/blood/requests")
      .wrap(RequirePermission::new(crate::security::policy::perm::BLOOD_REQUEST_CREATE))
      .route(web::post().to(crate::http::handlers::requests::create::create)))
    .service(web::resource("/api/v1/blood/crossmatch:perform")
      .wrap(RequirePermission::new(crate::security::policy::perm::BLOOD_CROSSMATCH_PERFORM))
      .route(web::post().to(crate::http::handlers::crossmatch::perform::perform)))
    .service(web::resource("/api/v1/blood/issues:release")
      .wrap(RequirePermission::new(crate::security::policy::perm::BLOOD_ISSUE_RELEASE))
      .route(web::post().to(crate::http::handlers::issues::release::release)))
}
