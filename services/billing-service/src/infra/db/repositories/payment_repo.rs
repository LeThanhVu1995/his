use sqlx::Row;
use uuid::Uuid;
use crate::domain::entities::payment::Payment;
use crate::infra::db::pool::PgPool;
use app_core::prelude::*;

pub async fn create(db: &PgPool, payment: &Payment) -> Result<(), app_error::AppError> {
    sqlx::query(
        r#"INSERT INTO payments(id,invoice_id,pay_no,method,amount,currency,received_at)
           VALUES($1,$2,$3,$4,$5,$6,$7)"#
    )
    .bind(payment.id)
    .bind(payment.invoice_id)
    .bind(&payment.pay_no)
    .bind(&payment.method)
    .bind(payment.amount.to_string())
    .bind(&payment.currency)
    .bind(payment.received_at)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn find_by_id(db: &PgPool, id: Uuid) -> Result<Option<Payment>, app_error::AppError> {
    let row = sqlx::query(
        r#"SELECT id,invoice_id,pay_no,method,amount,currency,received_at,created_at,updated_at
           FROM payments WHERE id=$1"#
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| Payment {
        id: r.get("id"),
        invoice_id: r.get("invoice_id"),
        pay_no: r.get("pay_no"),
        method: r.get("method"),
        amount: r.get::<String, _>("amount").parse().unwrap_or_default(),
        currency: r.get("currency"),
        received_at: r.get("received_at"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}

pub async fn list_paged(
    db: &PgPool,
    invoice_id: Option<Uuid>,
    page: i64,
    size: i64,
) -> Result<(Vec<Payment>, i64), app_error::AppError> {
    let page = page.max(1);
    let size = size.clamp(1, 200);
    let offset = (page - 1) * size;

    let (rows, total) = if let Some(inv_id) = invoice_id {
        let r = sqlx::query(
            r#"SELECT id,invoice_id,pay_no,method,amount,currency,received_at,created_at,updated_at
               FROM payments WHERE invoice_id=$1 ORDER BY received_at DESC OFFSET $2 LIMIT $3"#
        )
        .bind(inv_id)
        .bind(offset)
        .bind(size)
        .fetch_all(db)
        .await?;
        let t = sqlx::query(r#"SELECT COUNT(1) FROM payments WHERE invoice_id=$1"#)
            .bind(inv_id)
            .fetch_one(db)
            .await?
            .get::<i64, _>(0);
        (r, t)
    } else {
        let r = sqlx::query(
            r#"SELECT id,invoice_id,pay_no,method,amount,currency,received_at,created_at,updated_at
               FROM payments ORDER BY received_at DESC OFFSET $1 LIMIT $2"#
        )
        .bind(offset)
        .bind(size)
        .fetch_all(db)
        .await?;
        let t = sqlx::query(r#"SELECT COUNT(1) FROM payments"#)
            .fetch_one(db)
            .await?
            .get::<i64, _>(0);
        (r, t)
    };

    let items: Vec<Payment> = rows.into_iter().map(|r| Payment {
        id: r.get("id"),
        invoice_id: r.get("invoice_id"),
        pay_no: r.get("pay_no"),
        method: r.get("method"),
        amount: r.get::<String, _>("amount").parse().unwrap_or_default(),
        currency: r.get("currency"),
        received_at: r.get("received_at"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect();

    Ok((items, total))
}
