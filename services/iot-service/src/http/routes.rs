use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;

pub fn api_scope()->Scope{
  web::scope("")
    .service(crate::http::handlers::health::healthz)
    .service(
      web::resource("/api/v1/iot/devices:upsert")
        .wrap(RequirePermission::new(crate::security::policy::perm::IOT_DEVICE_UPSERT))
        .route(web::post().to(crate::http::handlers::devices::upsert)))
    .service(
      web::resource("/api/v1/iot/vitals:ingest")
        .wrap(RequirePermission::new(crate::security::policy::perm::IOT_VITAL_INGEST))
        .route(web::post().to(crate::http::handlers::ingest_vitals::ingest)))
}
