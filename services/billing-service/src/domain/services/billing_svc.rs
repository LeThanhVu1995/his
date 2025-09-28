use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::charge::{Charge, PriceList, PriceItem};
use crate::domain::entities::invoice::Invoice;
use crate::domain::entities::invoice_item::InvoiceItem;
use crate::domain::entities::payment::{Payment, PaymentAllocation, Refund};
use crate::infra::db::repositories::{
    charge_repo::{ChargeRepo, PriceListRepo, PriceItemRepo},
    invoice_repo::InvoiceRepo,
    invoice_item_repo::InvoiceItemRepo,
    payment_repo::{PaymentRepo, PaymentAllocationRepo, RefundRepo},
};

pub struct BillingService<'a> {
    pub charge_repo: ChargeRepo<'a>,
    pub price_list_repo: PriceListRepo<'a>,
    pub price_item_repo: PriceItemRepo<'a>,
    pub invoice_repo: InvoiceRepo<'a>,
    pub invoice_item_repo: InvoiceItemRepo<'a>,
    pub payment_repo: PaymentRepo<'a>,
    pub payment_allocation_repo: PaymentAllocationRepo<'a>,
    pub refund_repo: RefundRepo<'a>,
}

impl<'a> BillingService<'a> {
    // Charge Management
    pub async fn create_charge(
        &self,
        encounter_id: Uuid,
        patient_id: Uuid,
        service_code: String,
        description: Option<String>,
        qty: f64,
        unit_price: f64,
    ) -> anyhow::Result<Uuid> {
        let charge_id = Uuid::new_v4();
        let amount = qty * unit_price;
        let now = Utc::now();

        let charge = Charge {
            charge_id,
            encounter_id,
            patient_id,
            service_code,
            description,
            qty,
            unit_price,
            amount,
            status: "PENDING".to_string(),
            charged_at: now,
            created_at: now,
            updated_at: now,
        };

        self.charge_repo.insert(&charge).await?;
        Ok(charge_id)
    }

    pub async fn get_charge(&self, charge_id: Uuid) -> anyhow::Result<Option<Charge>> {
        self.charge_repo.get(charge_id).await
    }

    pub async fn list_charges_by_encounter(&self, encounter_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<Charge>> {
        self.charge_repo.list_by_encounter(encounter_id, limit, offset).await
    }

    pub async fn update_charge_status(&self, charge_id: Uuid, status: &str) -> anyhow::Result<()> {
        self.charge_repo.update_status(charge_id, status).await
    }

    // Invoice Management
    pub async fn create_invoice_from_charges(
        &self,
        encounter_id: Uuid,
        patient_id: Uuid,
        currency: String,
        due_date: Option<chrono::NaiveDate>,
    ) -> anyhow::Result<Uuid> {
        let invoice_id = Uuid::new_v4();
        let now = Utc::now();

        // Get pending charges for this encounter
        let charges = self.charge_repo.list_pending_by_encounter(encounter_id).await?;

        if charges.is_empty() {
            return Err(anyhow::anyhow!("No pending charges found for encounter"));
        }

        // Calculate total amount
        let total_amount: f64 = charges.iter().map(|c| c.amount).sum();

        // Create invoice
        let invoice = Invoice {
            invoice_id,
            encounter_id,
            patient_id,
            status: "OPEN".to_string(),
            total_amount: total_amount.clone(),
            currency,
            issued_at: now,
            due_date,
            created_at: now,
            updated_at: now,
        };

        self.invoice_repo.insert(&invoice).await?;

        // Create invoice items from charges
        for charge in &charges {
            let item_id = Uuid::new_v4();
            let item = InvoiceItem {
                invoice_item_id: item_id,
                invoice_id,
                service_code: charge.service_code.clone(),
                description: charge.description.clone(),
                qty: charge.qty.clone(),
                unit_price: charge.unit_price.clone(),
                amount: charge.amount.clone(),
                created_at: now,
                updated_at: now,
            };
            self.invoice_item_repo.insert(&item).await?;
        }

        // Update charges status to INVOICED
        for charge in &charges {
            self.charge_repo.update_status(charge.charge_id, "INVOICED").await?;
        }

        Ok(invoice_id)
    }

    pub async fn get_invoice(&self, invoice_id: Uuid) -> anyhow::Result<Option<Invoice>> {
        self.invoice_repo.get(invoice_id).await
    }

    pub async fn get_invoice_with_items(&self, invoice_id: Uuid) -> anyhow::Result<Option<(Invoice, Vec<InvoiceItem>)>> {
        if let Some(invoice) = self.invoice_repo.get(invoice_id).await? {
            let items = self.invoice_item_repo.list_by_invoice(invoice_id).await?;
            Ok(Some((invoice, items)))
        } else {
            Ok(None)
        }
    }

    pub async fn list_invoices_by_encounter(&self, encounter_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<Invoice>> {
        self.invoice_repo.list_by_encounter(encounter_id, limit, offset).await
    }

    pub async fn update_invoice_status(&self, invoice_id: Uuid, status: &str) -> anyhow::Result<()> {
        self.invoice_repo.update_status(invoice_id, status).await
    }

    // Payment Management
    pub async fn create_payment(
        &self,
        invoice_id: Uuid,
        method_code: String,
        amount: f64,
        ref_no: Option<String>,
    ) -> anyhow::Result<Uuid> {
        let payment_id = Uuid::new_v4();
        let now = Utc::now();

        let payment = Payment {
            payment_id,
            invoice_id,
            method_code,
            amount,
            paid_at: now,
            ref_no,
            status: "COMPLETED".to_string(),
            created_at: now,
            updated_at: now,
        };

        self.payment_repo.insert(&payment).await?;

        // Update invoice status to PAID if fully paid
        self.update_invoice_status_if_fully_paid(invoice_id).await?;

        Ok(payment_id)
    }

    pub async fn get_payment(&self, payment_id: Uuid) -> anyhow::Result<Option<Payment>> {
        self.payment_repo.get(payment_id).await
    }

    pub async fn list_payments_by_invoice(&self, invoice_id: Uuid) -> anyhow::Result<Vec<Payment>> {
        self.payment_repo.list_by_invoice(invoice_id).await
    }

    // Refund Management
    pub async fn create_refund(
        &self,
        payment_id: Uuid,
        amount: f64,
        reason: Option<String>,
        ref_no: Option<String>,
    ) -> anyhow::Result<Uuid> {
        let refund_id = Uuid::new_v4();
        let now = Utc::now();

        let refund = Refund {
            refund_id,
            payment_id,
            amount,
            reason,
            refunded_at: now,
            ref_no,
            status: "PENDING".to_string(),
            created_at: now,
            updated_at: now,
        };

        self.refund_repo.insert(&refund).await?;
        Ok(refund_id)
    }

    pub async fn get_refund(&self, refund_id: Uuid) -> anyhow::Result<Option<Refund>> {
        self.refund_repo.get(refund_id).await
    }

    pub async fn list_refunds_by_payment(&self, payment_id: Uuid) -> anyhow::Result<Vec<Refund>> {
        self.refund_repo.list_by_payment(payment_id).await
    }

    // Price List Management
    pub async fn create_price_list(
        &self,
        facility_id: Uuid,
        code: String,
        name: String,
        currency: String,
        valid_from: Option<chrono::NaiveDate>,
        valid_to: Option<chrono::NaiveDate>,
    ) -> anyhow::Result<Uuid> {
        let price_list_id = Uuid::new_v4();
        let now = Utc::now();

        let price_list = PriceList {
            price_list_id,
            facility_id,
            code,
            name,
            currency,
            valid_from,
            valid_to,
            created_at: now,
            updated_at: now,
        };

        self.price_list_repo.insert(&price_list).await?;
        Ok(price_list_id)
    }

    pub async fn add_price_item(
        &self,
        price_list_id: Uuid,
        service_code: String,
        description: Option<String>,
        unit_price: f64,
    ) -> anyhow::Result<Uuid> {
        let price_item_id = Uuid::new_v4();
        let now = Utc::now();

        let price_item = PriceItem {
            price_item_id,
            price_list_id,
            service_code,
            description,
            unit_price,
            created_at: now,
            updated_at: now,
        };

        self.price_item_repo.insert(&price_item).await?;
        Ok(price_item_id)
    }

    pub async fn get_price_by_service(&self, price_list_id: Uuid, service_code: &str) -> anyhow::Result<Option<f64>> {
        if let Some(item) = self.price_item_repo.get_by_service_code(price_list_id, service_code).await? {
            Ok(Some(item.unit_price))
        } else {
            Ok(None)
        }
    }

    // Helper methods
    async fn update_invoice_status_if_fully_paid(&self, invoice_id: Uuid) -> anyhow::Result<()> {
        if let Some(invoice) = self.invoice_repo.get(invoice_id).await? {
            let payments = self.payment_repo.list_by_invoice(invoice_id).await?;
            let total_paid: f64 = payments.iter().map(|p| p.amount).sum();

            if total_paid >= invoice.total_amount {
                self.invoice_repo.update_status(invoice_id, "PAID").await?;
            }
        }
        Ok(())
    }
}
