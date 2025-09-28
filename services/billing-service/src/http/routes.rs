use actix_web::{web, Scope};
use crate::http::handlers::{
    health,
    charges::{list, create, update, get as get_charge},
    invoices::{list as invoice_list, create as invoice_create, issue, get as get_invoice},
    payments::{list as payment_list, create as payment_create, get as get_payment},
    refunds::{list as refund_list, create as refund_create},
};

pub fn api_scope() -> Scope {
    web::scope("")
        .route("/healthz", web::get().to(health::healthz))

        // Charges
        .service(list::list_charges)
        .service(create::create_charge)
        .service(update::update_charge)
        .service(get_charge::get_charge)

        // Invoices
        .service(invoice_list::list_invoices)
        .service(invoice_create::create_invoice)
        .service(issue::issue_invoice)
        .service(get_invoice::get_invoice)

        // Payments
        .service(payment_list::list_payments)
        .service(payment_create::create_payment)
        .service(get_payment::get_payment)

        // Refunds
        .service(refund_list::list_refunds)
        .service(refund_create::create_refund)
}
