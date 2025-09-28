use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::entities::invoice::Invoice;

#[derive(Clone)]
pub struct InvoiceRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> InvoiceRepo<'a> {
    pub async fn insert(&self, invoice: &Invoice) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bill_invoice(invoice_id, encounter_id, patient_id, status, total_amount, currency, issued_at, due_date, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
        )
        .bind(invoice.invoice_id)
        .bind(invoice.encounter_id)
        .bind(invoice.patient_id)
        .bind(&invoice.status)
        .bind(&invoice.total_amount)
        .bind(&invoice.currency)
        .bind(invoice.issued_at)
        .bind(invoice.due_date)
        .bind(invoice.created_at)
        .bind(invoice.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, invoice_id: Uuid) -> anyhow::Result<Option<Invoice>> {
        Ok(sqlx::query_as::<_, Invoice>(
            "SELECT invoice_id, encounter_id, patient_id, status, total_amount, currency, issued_at, due_date, created_at, updated_at
             FROM bill_invoice WHERE invoice_id = $1"
        )
        .bind(invoice_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_encounter(&self, encounter_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<Invoice>> {
        Ok(sqlx::query_as::<_, Invoice>(
            "SELECT invoice_id, encounter_id, patient_id, status, total_amount, currency, issued_at, due_date, created_at, updated_at
             FROM bill_invoice WHERE encounter_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(encounter_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn list_by_patient(&self, patient_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<Invoice>> {
        Ok(sqlx::query_as::<_, Invoice>(
            "SELECT invoice_id, encounter_id, patient_id, status, total_amount, currency, issued_at, due_date, created_at, updated_at
             FROM bill_invoice WHERE patient_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn list_by_status(&self, status: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<Invoice>> {
        Ok(sqlx::query_as::<_, Invoice>(
            "SELECT invoice_id, encounter_id, patient_id, status, total_amount, currency, issued_at, due_date, created_at, updated_at
             FROM bill_invoice WHERE status = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn update_status(&self, invoice_id: Uuid, status: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE bill_invoice SET status = $2, updated_at = NOW() WHERE invoice_id = $1")
            .bind(invoice_id)
            .bind(status)
            .execute(self.db)
            .await?;
        Ok(())
    }

    pub async fn update_total_amount(&self, invoice_id: Uuid, total_amount: f64) -> anyhow::Result<()> {
        sqlx::query("UPDATE bill_invoice SET total_amount = $2, updated_at = NOW() WHERE invoice_id = $1")
            .bind(invoice_id)
            .bind(total_amount)
            .execute(self.db)
            .await?;
        Ok(())
    }
}
