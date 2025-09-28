use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;

pub fn api_scope(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/healthz", web::get().to(crate::http::handlers::health::healthz))

            // Patient routes
            .service(
                web::resource("/api/v1/patients")
                    .wrap(RequirePermission::new("his.emr.patient.read"))
                    .route(web::post().to(crate::http::handlers::patient::create_patient))
                    .route(web::get().to(crate::http::handlers::patient::list_patients))
            )
            .service(
                web::resource("/api/v1/patients/{patient_id}")
                    .wrap(RequirePermission::new("his.emr.patient.read"))
                    .route(web::get().to(crate::http::handlers::patient::get_patient))
                    .route(web::put().to(crate::http::handlers::patient::update_patient))
                    .route(web::delete().to(crate::http::handlers::patient::delete_patient))
            )
            .service(
                web::resource("/api/v1/patients/by-code/{code}")
                    .wrap(RequirePermission::new("his.emr.patient.read"))
                    .route(web::get().to(crate::http::handlers::patient::get_patient_by_code))
            )
            .service(
                web::resource("/api/v1/patients/search")
                    .wrap(RequirePermission::new("his.emr.patient.read"))
                    .route(web::post().to(crate::http::handlers::patient::search_patients))
            )

            // Patient Identifier routes
            .service(
                web::resource("/api/v1/patients/{patient_id}/identifiers")
                    .wrap(RequirePermission::new("his.emr.patient.read"))
                    .route(web::post().to(crate::http::handlers::patient::create_patient_identifier))
                    .route(web::get().to(crate::http::handlers::patient::get_patient_identifiers))
            )

            // Patient Contact routes
            .service(
                web::resource("/api/v1/patients/{patient_id}/contacts")
                    .wrap(RequirePermission::new("his.emr.patient.read"))
                    .route(web::post().to(crate::http::handlers::patient::create_patient_contact))
                    .route(web::get().to(crate::http::handlers::patient::get_patient_contacts))
            )

            // Episode of Care routes
            .service(
                web::resource("/api/v1/patients/{patient_id}/episodes")
                    .wrap(RequirePermission::new("his.emr.episode.read"))
                    .route(web::post().to(crate::http::handlers::patient::create_episode))
                    .route(web::get().to(crate::http::handlers::patient::get_patient_episodes))
            )
            .service(
                web::resource("/api/v1/episodes/{episode_id}/close")
                    .wrap(RequirePermission::new("his.emr.episode.write"))
                    .route(web::post().to(crate::http::handlers::patient::close_episode))
            )

            // Encounter routes
            .service(
                web::resource("/api/v1/encounters")
                    .wrap(RequirePermission::new("his.emr.encounter.read"))
                    .route(web::post().to(crate::http::handlers::encounter::create_encounter))
            )
            .service(
                web::resource("/api/v1/encounters/{encounter_id}")
                    .wrap(RequirePermission::new("his.emr.encounter.read"))
                    .route(web::get().to(crate::http::handlers::encounter::get_encounter))
                    .route(web::put().to(crate::http::handlers::encounter::update_encounter))
                    .route(web::post().to(crate::http::handlers::encounter::end_encounter))
            )
            .service(
                web::resource("/api/v1/patients/{patient_id}/encounters")
                    .wrap(RequirePermission::new("his.emr.encounter.read"))
                    .route(web::get().to(crate::http::handlers::encounter::list_patient_encounters))
            )
            .service(
                web::resource("/api/v1/facilities/{facility_id}/encounters")
                    .wrap(RequirePermission::new("his.emr.encounter.read"))
                    .route(web::get().to(crate::http::handlers::encounter::list_encounters_by_facility))
            )

            // Clinical Notes routes
            .service(
                web::resource("/api/v1/clinical-notes")
                    .wrap(RequirePermission::new("his.emr.note.read"))
                    .route(web::post().to(crate::http::handlers::encounter::create_clinical_note))
            )
            .service(
                web::resource("/api/v1/encounters/{encounter_id}/notes")
                    .wrap(RequirePermission::new("his.emr.note.read"))
                    .route(web::get().to(crate::http::handlers::encounter::get_encounter_notes))
            )
            .service(
                web::resource("/api/v1/clinical-notes/{note_id}")
                    .wrap(RequirePermission::new("his.emr.note.write"))
                    .route(web::put().to(crate::http::handlers::encounter::update_clinical_note))
                    .route(web::delete().to(crate::http::handlers::encounter::delete_clinical_note))
            )

            // Problem List routes
            .service(
                web::resource("/api/v1/problems")
                    .wrap(RequirePermission::new("his.emr.problem.read"))
                    .route(web::post().to(crate::http::handlers::problem::create_problem))
            )
            .service(
                web::resource("/api/v1/problems/{problem_id}")
                    .wrap(RequirePermission::new("his.emr.problem.read"))
                    .route(web::get().to(crate::http::handlers::problem::get_problem))
                    .route(web::put().to(crate::http::handlers::problem::update_problem))
                    .route(web::delete().to(crate::http::handlers::problem::delete_problem))
            )
            .service(
                web::resource("/api/v1/problems/{problem_id}/resolve")
                    .wrap(RequirePermission::new("his.emr.problem.write"))
                    .route(web::post().to(crate::http::handlers::problem::resolve_problem))
            )
            .service(
                web::resource("/api/v1/patients/{patient_id}/problems")
                    .wrap(RequirePermission::new("his.emr.problem.read"))
                    .route(web::get().to(crate::http::handlers::problem::list_patient_problems))
            )

            // Allergy Intolerance routes
            .service(
                web::resource("/api/v1/allergies")
                    .wrap(RequirePermission::new("his.emr.allergy.read"))
                    .route(web::post().to(crate::http::handlers::allergy::create_allergy))
            )
            .service(
                web::resource("/api/v1/allergies/{allergy_id}")
                    .wrap(RequirePermission::new("his.emr.allergy.read"))
                    .route(web::get().to(crate::http::handlers::allergy::get_allergy))
                    .route(web::put().to(crate::http::handlers::allergy::update_allergy))
                    .route(web::delete().to(crate::http::handlers::allergy::delete_allergy))
            )
            .service(
                web::resource("/api/v1/patients/{patient_id}/allergies")
                    .wrap(RequirePermission::new("his.emr.allergy.read"))
                    .route(web::get().to(crate::http::handlers::allergy::list_patient_allergies))
            )

            // Medication Statement routes
            .service(
                web::resource("/api/v1/medications")
                    .wrap(RequirePermission::new("his.emr.medication.read"))
                    .route(web::post().to(crate::http::handlers::allergy::create_medication))
            )
            .service(
                web::resource("/api/v1/medications/{medication_id}")
                    .wrap(RequirePermission::new("his.emr.medication.read"))
                    .route(web::get().to(crate::http::handlers::allergy::get_medication))
                    .route(web::put().to(crate::http::handlers::allergy::update_medication))
                    .route(web::delete().to(crate::http::handlers::allergy::delete_medication))
            )
            .service(
                web::resource("/api/v1/patients/{patient_id}/medications")
                    .wrap(RequirePermission::new("his.emr.medication.read"))
                    .route(web::get().to(crate::http::handlers::allergy::list_patient_medications))
            )

            // Vital Sign routes
            .service(
                web::resource("/api/v1/vital-signs")
                    .wrap(RequirePermission::new("his.emr.vital.read"))
                    .route(web::post().to(crate::http::handlers::vital::create_vital_sign_record))
            )
            .service(
                web::resource("/api/v1/vital-signs/{record_id}")
                    .wrap(RequirePermission::new("his.emr.vital.read"))
                    .route(web::get().to(crate::http::handlers::vital::get_vital_sign_record))
                    .route(web::put().to(crate::http::handlers::vital::update_vital_sign_record))
                    .route(web::delete().to(crate::http::handlers::vital::delete_vital_sign_record))
            )
            .service(
                web::resource("/api/v1/patients/{patient_id}/vital-signs")
                    .wrap(RequirePermission::new("his.emr.vital.read"))
                    .route(web::get().to(crate::http::handlers::vital::list_patient_vital_signs))
            )

            // Vital Sign Item routes
            .service(
                web::resource("/api/v1/vital-signs/{record_id}/items")
                    .wrap(RequirePermission::new("his.emr.vital.read"))
                    .route(web::post().to(crate::http::handlers::vital::create_vital_sign_item))
                    .route(web::get().to(crate::http::handlers::vital::get_vital_sign_items))
            )
            .service(
                web::resource("/api/v1/vital-sign-items/{item_id}")
                    .wrap(RequirePermission::new("his.emr.vital.write"))
                    .route(web::put().to(crate::http::handlers::vital::update_vital_sign_item))
                    .route(web::delete().to(crate::http::handlers::vital::delete_vital_sign_item))
            )

            // Observation routes
            .service(
                web::resource("/api/v1/observations")
                    .wrap(RequirePermission::new("his.emr.observation.read"))
                    .route(web::post().to(crate::http::handlers::vital::create_observation))
            )
            .service(
                web::resource("/api/v1/observations/{observation_id}")
                    .wrap(RequirePermission::new("his.emr.observation.read"))
                    .route(web::get().to(crate::http::handlers::vital::get_observation))
                    .route(web::put().to(crate::http::handlers::vital::update_observation))
                    .route(web::delete().to(crate::http::handlers::vital::delete_observation))
            )
            .service(
                web::resource("/api/v1/patients/{patient_id}/observations")
                    .wrap(RequirePermission::new("his.emr.observation.read"))
                    .route(web::get().to(crate::http::handlers::vital::list_patient_observations))
            )

            // Clinical Order routes
            .service(
                web::resource("/api/v1/orders")
                    .wrap(RequirePermission::new("his.emr.order.read"))
                    .route(web::post().to(crate::http::handlers::order::create_order))
            )
            .service(
                web::resource("/api/v1/orders/{order_id}")
                    .wrap(RequirePermission::new("his.emr.order.read"))
                    .route(web::get().to(crate::http::handlers::order::get_order))
                    .route(web::put().to(crate::http::handlers::order::update_order))
                    .route(web::delete().to(crate::http::handlers::order::delete_order))
            )
            .service(
                web::resource("/api/v1/orders/{order_id}/complete")
                    .wrap(RequirePermission::new("his.emr.order.write"))
                    .route(web::post().to(crate::http::handlers::order::complete_order))
            )
            .service(
                web::resource("/api/v1/orders/{order_id}/cancel")
                    .wrap(RequirePermission::new("his.emr.order.write"))
                    .route(web::post().to(crate::http::handlers::order::cancel_order))
            )
            .service(
                web::resource("/api/v1/patients/{patient_id}/orders")
                    .wrap(RequirePermission::new("his.emr.order.read"))
                    .route(web::get().to(crate::http::handlers::order::list_patient_orders))
            )
            .service(
                web::resource("/api/v1/encounters/{encounter_id}/orders")
                    .wrap(RequirePermission::new("his.emr.order.read"))
                    .route(web::get().to(crate::http::handlers::order::list_encounter_orders))
            )
    );
}
