use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::entities::payment::{Payment, PaymentAllocation, Refund};

#[derive(Clone)]
pub struct PaymentRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> PaymentRepo<'a> {
    pub async fn insert(&self, payment: &Payment) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bill_payment(payment_id, invoice_id, method_code, amount, paid_at, ref_no, status, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(payment.payment_id)
        .bind(payment.invoice_id)
        .bind(&payment.method_code)
        .bind(&payment.amount)
        .bind(payment.paid_at)
        .bind(payment.ref_no.as_ref())
        .bind(&payment.status)
        .bind(payment.created_at)
        .bind(payment.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, payment_id: Uuid) -> anyhow::Result<Option<Payment>> {
        Ok(sqlx::query_as::<_, Payment>(
            "SELECT payment_id, invoice_id, method_code, amount, paid_at, ref_no, status, created_at, updated_at
             FROM bill_payment WHERE payment_id = $1"
        )
        .bind(payment_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_invoice(&self, invoice_id: Uuid) -> anyhow::Result<Vec<Payment>> {
        Ok(sqlx::query_as::<_, Payment>(
            "SELECT payment_id, invoice_id, method_code, amount, paid_at, ref_no, status, created_at, updated_at
             FROM bill_payment WHERE invoice_id = $1 ORDER BY paid_at DESC"
        )
        .bind(invoice_id)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn update_status(&self, payment_id: Uuid, status: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE bill_payment SET status = $2, updated_at = NOW() WHERE payment_id = $1")
            .bind(payment_id)
            .bind(status)
            .execute(self.db)
            .await?;
        Ok(())
    }
}

// Payment Allocation Repository
#[derive(Clone)]
pub struct PaymentAllocationRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> PaymentAllocationRepo<'a> {
    pub async fn insert(&self, allocation: &PaymentAllocation) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bill_payment_allocation(allocation_id, payment_id, invoice_item_id, amount, created_at)
             VALUES($1, $2, $3, $4, $5)"
        )
        .bind(allocation.allocation_id)
        .bind(allocation.payment_id)
        .bind(allocation.invoice_item_id)
        .bind(&allocation.amount)
        .bind(allocation.created_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_by_payment(&self, payment_id: Uuid) -> anyhow::Result<Vec<PaymentAllocation>> {
        Ok(sqlx::query_as::<_, PaymentAllocation>(
            "SELECT allocation_id, payment_id, invoice_item_id, amount, created_at
             FROM bill_payment_allocation WHERE payment_id = $1 ORDER BY created_at"
        )
        .bind(payment_id)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn list_by_invoice_item(&self, invoice_item_id: Uuid) -> anyhow::Result<Vec<PaymentAllocation>> {
        Ok(sqlx::query_as::<_, PaymentAllocation>(
            "SELECT allocation_id, payment_id, invoice_item_id, amount, created_at
             FROM bill_payment_allocation WHERE invoice_item_id = $1 ORDER BY created_at"
        )
        .bind(invoice_item_id)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn delete_by_payment(&self, payment_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM bill_payment_allocation WHERE payment_id = $1")
            .bind(payment_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}

// Refund Repository
#[derive(Clone)]
pub struct RefundRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> RefundRepo<'a> {
    pub async fn insert(&self, refund: &Refund) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bill_refund(refund_id, payment_id, amount, reason, refunded_at, ref_no, status, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(refund.refund_id)
        .bind(refund.payment_id)
        .bind(&refund.amount)
        .bind(refund.reason.as_ref())
        .bind(refund.refunded_at)
        .bind(refund.ref_no.as_ref())
        .bind(&refund.status)
        .bind(refund.created_at)
        .bind(refund.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, refund_id: Uuid) -> anyhow::Result<Option<Refund>> {
        Ok(sqlx::query_as::<_, Refund>(
            "SELECT refund_id, payment_id, amount, reason, refunded_at, ref_no, status, created_at, updated_at
             FROM bill_refund WHERE refund_id = $1"
        )
        .bind(refund_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_payment(&self, payment_id: Uuid) -> anyhow::Result<Vec<Refund>> {
        Ok(sqlx::query_as::<_, Refund>(
            "SELECT refund_id, payment_id, amount, reason, refunded_at, ref_no, status, created_at, updated_at
             FROM bill_refund WHERE payment_id = $1 ORDER BY refunded_at DESC"
        )
        .bind(payment_id)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn update_status(&self, refund_id: Uuid, status: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE bill_refund SET status = $2, updated_at = NOW() WHERE refund_id = $1")
            .bind(refund_id)
            .bind(status)
            .execute(self.db)
            .await?;
        Ok(())
    }
}
