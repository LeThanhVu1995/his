use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;
use crate::security::policy::perm;

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(crate::http::handlers::health::healthz))

        // Device Management
        .service(
            web::resource("/api/v1/iot/devices:upsert")
                .wrap(RequirePermission::new(perm::IOT_DEVICE_UPSERT))
                .route(web::post().to(crate::http::handlers::devices::upsert))
        )
        .service(
            web::resource("/api/v1/iot/devices/{id}")
                .wrap(RequirePermission::new(perm::IOT_DEVICE_GET))
                .route(web::get().to(crate::http::handlers::devices::get_device))
        )
        .service(
            web::resource("/api/v1/iot/devices/{id}")
                .wrap(RequirePermission::new(perm::IOT_DEVICE_UPDATE))
                .route(web::put().to(crate::http::handlers::devices::update_device))
        )
        .service(
            web::resource("/api/v1/iot/devices/{id}")
                .wrap(RequirePermission::new(perm::IOT_DEVICE_DELETE))
                .route(web::delete().to(crate::http::handlers::devices::delete_device))
        )
        .service(
            web::resource("/api/v1/iot/devices")
                .wrap(RequirePermission::new(perm::IOT_DEVICE_LIST))
                .route(web::get().to(crate::http::handlers::devices::list_devices))
        )

        // Vital Signs Management
        .service(
            web::resource("/api/v1/iot/vital-signs:create")
                .wrap(RequirePermission::new(perm::IOT_VITAL_CREATE))
                .route(web::post().to(crate::http::handlers::vital_signs::create_vital_signs))
        )
        .service(
            web::resource("/api/v1/iot/vital-signs/{id}")
                .wrap(RequirePermission::new(perm::IOT_VITAL_GET))
                .route(web::get().to(crate::http::handlers::vital_signs::get_vital_signs))
        )
        .service(
            web::resource("/api/v1/iot/vital-signs")
                .wrap(RequirePermission::new(perm::IOT_VITAL_LIST))
                .route(web::get().to(crate::http::handlers::vital_signs::list_vital_signs))
        )

        // Observations Management
        .service(
            web::resource("/api/v1/iot/observations:create")
                .wrap(RequirePermission::new(perm::IOT_OBSERVATION_CREATE))
                .route(web::post().to(crate::http::handlers::observations::create_observation))
        )
        .service(
            web::resource("/api/v1/iot/observations/{id}")
                .wrap(RequirePermission::new(perm::IOT_OBSERVATION_GET))
                .route(web::get().to(crate::http::handlers::observations::get_observation))
        )
        .service(
            web::resource("/api/v1/iot/observations/{id}")
                .wrap(RequirePermission::new(perm::IOT_OBSERVATION_UPDATE))
                .route(web::put().to(crate::http::handlers::observations::update_observation))
        )
        .service(
            web::resource("/api/v1/iot/observations")
                .wrap(RequirePermission::new(perm::IOT_OBSERVATION_LIST))
                .route(web::get().to(crate::http::handlers::observations::list_observations))
        )

        // Device Readings Management
        .service(
            web::resource("/api/v1/iot/device-readings:create")
                .wrap(RequirePermission::new(perm::IOT_READING_CREATE))
                .route(web::post().to(crate::http::handlers::device_readings::create_device_reading))
        )
        .service(
            web::resource("/api/v1/iot/device-readings/{id}")
                .wrap(RequirePermission::new(perm::IOT_READING_GET))
                .route(web::get().to(crate::http::handlers::device_readings::get_device_reading))
        )
        .service(
            web::resource("/api/v1/iot/device-readings")
                .wrap(RequirePermission::new(perm::IOT_READING_LIST))
                .route(web::get().to(crate::http::handlers::device_readings::list_device_readings))
        )

        // Data Ingestion (Legacy endpoints for backward compatibility)
        .service(
            web::resource("/api/v1/iot/vitals:ingest")
                .wrap(RequirePermission::new(perm::IOT_VITAL_INGEST))
                .route(web::post().to(crate::http::handlers::ingest_vitals::ingest))
        )
}
