use crate::domain::services::billing_svc::BillingService;
use crate::infra::db::repositories::{
    charge_repo::{ChargeRepo, PriceListRepo, PriceItemRepo},
    invoice_repo::InvoiceRepo,
    invoice_item_repo::InvoiceItemRepo,
    payment_repo::{PaymentRepo, PaymentAllocationRepo, RefundRepo},
};

pub fn create_billing_service(db: &sqlx::Pool<sqlx::Postgres>) -> BillingService {
    BillingService {
        charge_repo: ChargeRepo { db },
        price_list_repo: PriceListRepo { db },
        price_item_repo: PriceItemRepo { db },
        invoice_repo: InvoiceRepo { db },
        invoice_item_repo: InvoiceItemRepo { db },
        payment_repo: PaymentRepo { db },
        payment_allocation_repo: PaymentAllocationRepo { db },
        refund_repo: RefundRepo { db },
    }
}
