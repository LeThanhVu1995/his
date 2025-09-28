use actix_web::{web, Scope};

pub fn api_scope() -> Scope {
    web::scope("")
        .service(crate::http::handlers::health::healthz)

        // Donor CRUD handlers
        .service(crate::http::handlers::donors::create::create_donor)
        .service(crate::http::handlers::donors::list::list_donors)
        .service(crate::http::handlers::donors::get::get_donor)
        .service(crate::http::handlers::donors::update::update_donor)

        // Donation CRUD handlers
        .service(crate::http::handlers::donations::create::create_donation)
        .service(crate::http::handlers::donations::list::list_donations)
        .service(crate::http::handlers::donations::get::get_donation)

        // Blood Unit handlers
        .service(crate::http::handlers::blood_units::search::search_units)
        .service(crate::http::handlers::blood_units::compatibility::check_compatibility)
        .service(crate::http::handlers::blood_units::get::get_unit)
        .service(crate::http::handlers::blood_units::update_status::update_unit_status)

        // Adverse Event handlers
        .service(crate::http::handlers::adverse_events::report::report_adverse_event)
        .service(crate::http::handlers::adverse_events::list::list_adverse_events)
        .service(crate::http::handlers::adverse_events::get::get_adverse_event)

        // Crossmatch handlers
        .service(crate::http::handlers::crossmatch::perform::perform_crossmatch)
        .service(crate::http::handlers::crossmatch::find_compatible::find_compatible_units)

        // Issue handlers
        .service(crate::http::handlers::issues::issue::issue_blood_unit)
        .service(crate::http::handlers::issues::r#return::return_blood_unit)
        .service(crate::http::handlers::issues::list::list_issues)

        // Blood Request handlers
        .service(crate::http::handlers::blood_requests::create::create_blood_request)
        .service(crate::http::handlers::blood_requests::list::list_blood_requests)
        .service(crate::http::handlers::blood_requests::get::get_blood_request)
        .service(crate::http::handlers::blood_requests::update_status::update_blood_request_status)
}
