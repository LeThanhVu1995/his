use sqlx::Row;
use uuid::Uuid;
use crate::domain::entities::invoice_item::InvoiceItem;
use crate::infra::db::pool::PgPool;
use app_core::prelude::*;

pub async fn insert_many(db: &PgPool, items: &[InvoiceItem]) -> Result<(), app_error::AppError> {
    for item in items {
        sqlx::query(
            r#"INSERT INTO invoice_items(id,invoice_id,charge_id,code,name,qty,unit_price,amount)
               VALUES($1,$2,$3,$4,$5,$6,$7,$8)"#
        )
        .bind(item.id)
        .bind(item.invoice_id)
        .bind(item.charge_id)
        .bind(&item.code)
        .bind(&item.name)
        .bind(item.qty.to_string())
        .bind(item.unit_price.to_string())
        .bind(item.amount.to_string())
        .execute(db)
        .await?;
    }
    Ok(())
}

pub async fn list_by_invoice(db: &PgPool, invoice_id: Uuid) -> Result<Vec<InvoiceItem>, app_error::AppError> {
    let rows = sqlx::query(
        r#"SELECT id,invoice_id,charge_id,code,name,qty,unit_price,amount,created_at,updated_at
           FROM invoice_items WHERE invoice_id=$1 ORDER BY created_at"#
    )
    .bind(invoice_id)
    .fetch_all(db)
    .await?;

    let items: Vec<InvoiceItem> = rows.into_iter().map(|r| InvoiceItem {
        id: r.get("id"),
        invoice_id: r.get("invoice_id"),
        charge_id: r.get("charge_id"),
        code: r.get("code"),
        name: r.get("name"),
        qty: r.get::<String, _>("qty").parse().unwrap_or_default(),
        unit_price: r.get::<String, _>("unit_price").parse().unwrap_or_default(),
        amount: r.get::<String, _>("amount").parse().unwrap_or_default(),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect();

    Ok(items)
}

