use sqlx::Row;
use uuid::Uuid;
use crate::domain::entities::invoice::Invoice;
use crate::infra::db::pool::PgPool;
use app_core::prelude::*;

pub async fn create(db: &PgPool, invoice: &Invoice) -> Result<(), app_error::AppError> {
    sqlx::query(
        r#"INSERT INTO invoices(id,invoice_no,patient_id,encounter_id,subtotal,discount,tax,total,status,note)
           VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#
    )
    .bind(invoice.id)
    .bind(&invoice.invoice_no)
    .bind(invoice.patient_id)
    .bind(invoice.encounter_id)
    .bind(invoice.subtotal.to_string())
    .bind(invoice.discount.to_string())
    .bind(invoice.tax.to_string())
    .bind(invoice.total.to_string())
    .bind(&invoice.status)
    .bind(&invoice.note)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn find_by_id(db: &PgPool, id: Uuid) -> Result<Option<Invoice>, app_error::AppError> {
    let row = sqlx::query(
        r#"SELECT id,invoice_no,patient_id,encounter_id,subtotal,discount,tax,total,status,note,created_at,updated_at
           FROM invoices WHERE id=$1"#
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| Invoice {
        id: r.get("id"),
        invoice_no: r.get("invoice_no"),
        patient_id: r.get("patient_id"),
        encounter_id: r.get("encounter_id"),
        subtotal: r.get::<String, _>("subtotal").parse().unwrap_or_default(),
        discount: r.get::<String, _>("discount").parse().unwrap_or_default(),
        tax: r.get::<String, _>("tax").parse().unwrap_or_default(),
        total: r.get::<String, _>("total").parse().unwrap_or_default(),
        status: r.get("status"),
        note: r.get("note"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}

pub async fn update_status(db: &PgPool, id: Uuid, status: &str) -> Result<Option<Invoice>, app_error::AppError> {
    let row = sqlx::query(
        r#"UPDATE invoices SET status=$2, updated_at=NOW()
           WHERE id=$1
           RETURNING id,invoice_no,patient_id,encounter_id,subtotal,discount,tax,total,status,note,created_at,updated_at"#
    )
    .bind(id)
    .bind(status)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| Invoice {
        id: r.get("id"),
        invoice_no: r.get("invoice_no"),
        patient_id: r.get("patient_id"),
        encounter_id: r.get("encounter_id"),
        subtotal: r.get::<String, _>("subtotal").parse().unwrap_or_default(),
        discount: r.get::<String, _>("discount").parse().unwrap_or_default(),
        tax: r.get::<String, _>("tax").parse().unwrap_or_default(),
        total: r.get::<String, _>("total").parse().unwrap_or_default(),
        status: r.get("status"),
        note: r.get("note"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}

pub async fn list_paged(
    db: &PgPool,
    encounter_id: Option<Uuid>,
    status: Option<&str>,
    page: i64,
    size: i64,
) -> Result<(Vec<Invoice>, i64), app_error::AppError> {
    let page = page.max(1);
    let size = size.clamp(1, 200);
    let offset = (page - 1) * size;

    let (rows, total) = if let Some(e_id) = encounter_id {
        if let Some(s) = status {
            let r = sqlx::query(
                r#"SELECT id,invoice_no,patient_id,encounter_id,subtotal,discount,tax,total,status,note,created_at,updated_at
                   FROM invoices WHERE encounter_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#
            )
            .bind(e_id)
            .bind(s)
            .bind(offset)
            .bind(size)
            .fetch_all(db)
            .await?;
            let t = sqlx::query(r#"SELECT COUNT(1) FROM invoices WHERE encounter_id=$1 AND status=$2"#)
                .bind(e_id)
                .bind(s)
                .fetch_one(db)
                .await?
                .get::<i64, _>(0);
            (r, t)
        } else {
            let r = sqlx::query(
                r#"SELECT id,invoice_no,patient_id,encounter_id,subtotal,discount,tax,total,status,note,created_at,updated_at
                   FROM invoices WHERE encounter_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#
            )
            .bind(e_id)
            .bind(offset)
            .bind(size)
            .fetch_all(db)
            .await?;
            let t = sqlx::query(r#"SELECT COUNT(1) FROM invoices WHERE encounter_id=$1"#)
                .bind(e_id)
                .fetch_one(db)
                .await?
                .get::<i64, _>(0);
            (r, t)
        }
    } else {
        let r = sqlx::query(
            r#"SELECT id,invoice_no,patient_id,encounter_id,subtotal,discount,tax,total,status,note,created_at,updated_at
               FROM invoices ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
        )
        .bind(offset)
        .bind(size)
        .fetch_all(db)
        .await?;
        let t = sqlx::query(r#"SELECT COUNT(1) FROM invoices"#)
            .fetch_one(db)
            .await?
            .get::<i64, _>(0);
        (r, t)
    };

    let items: Vec<Invoice> = rows.into_iter().map(|r| Invoice {
        id: r.get("id"),
        invoice_no: r.get("invoice_no"),
        patient_id: r.get("patient_id"),
        encounter_id: r.get("encounter_id"),
        subtotal: r.get::<String, _>("subtotal").parse().unwrap_or_default(),
        discount: r.get::<String, _>("discount").parse().unwrap_or_default(),
        tax: r.get::<String, _>("tax").parse().unwrap_or_default(),
        total: r.get::<String, _>("total").parse().unwrap_or_default(),
        status: r.get("status"),
        note: r.get("note"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect();

    Ok((items, total))
}
