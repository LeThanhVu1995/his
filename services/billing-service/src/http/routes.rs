use actix_web::{web, Scope};
use crate::http::handlers::{
    health,
    charges::{list, create, update},
    invoices::{list as invoice_list, create as invoice_create, issue},
    payments::{list as payment_list, create as payment_create},
};

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(health::healthz))
        // Charges
        // .service(
        //     web::resource("/api/v1/charges")
        //         .route(web::get().to(list::list_charges))
        // )
        // .service(
        //     web::resource("/api/v1/charges:create")
        //         .route(web::post().to(create::create_charge))
        // )
        // .service(
        //     web::resource("/api/v1/charges/{id}")
        //         .route(web::put().to(update::update_charge))
        // )
        // // Invoices
        // .service(
        //     web::resource("/api/v1/invoices")
        //         .route(web::get().to(invoice_list::list_invoices))
        // )
        // .service(
        //     web::resource("/api/v1/invoices:create")
        //         .route(web::post().to(invoice_create::create_invoice))
        // )
        // .service(
        //     web::resource("/api/v1/invoices/{id}:issue")
        //         .route(web::put().to(issue::issue_invoice))
        // )
        // // Payments
        // .service(
        //     web::resource("/api/v1/payments")
        //         .route(web::get().to(payment_list::list_payments))
        // )
        // .service(
        //     web::resource("/api/v1/payments:create")
        //         .route(web::post().to(payment_create::create_payment))
        // )
}
