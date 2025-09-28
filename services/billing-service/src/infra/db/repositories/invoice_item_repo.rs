use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::entities::invoice_item::InvoiceItem;

#[derive(Clone)]
pub struct InvoiceItemRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> InvoiceItemRepo<'a> {
    pub async fn insert(&self, item: &InvoiceItem) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bill_invoice_item(invoice_item_id, invoice_id, service_code, description, qty, unit_price, amount, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(item.invoice_item_id)
        .bind(item.invoice_id)
        .bind(&item.service_code)
        .bind(item.description.as_ref())
        .bind(&item.qty)
        .bind(&item.unit_price)
        .bind(&item.amount)
        .bind(item.created_at)
        .bind(item.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, invoice_item_id: Uuid) -> anyhow::Result<Option<InvoiceItem>> {
        Ok(sqlx::query_as::<_, InvoiceItem>(
            "SELECT invoice_item_id, invoice_id, service_code, description, qty, unit_price, amount, created_at, updated_at
             FROM bill_invoice_item WHERE invoice_item_id = $1"
        )
        .bind(invoice_item_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_invoice(&self, invoice_id: Uuid) -> anyhow::Result<Vec<InvoiceItem>> {
        Ok(sqlx::query_as::<_, InvoiceItem>(
            "SELECT invoice_item_id, invoice_id, service_code, description, qty, unit_price, amount, created_at, updated_at
             FROM bill_invoice_item WHERE invoice_id = $1 ORDER BY created_at"
        )
        .bind(invoice_id)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn delete_by_invoice(&self, invoice_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM bill_invoice_item WHERE invoice_id = $1")
            .bind(invoice_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}
