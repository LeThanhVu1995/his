use sqlx::Row;
use uuid::Uuid;
use crate::domain::entities::charge::Charge;
use crate::infra::db::pool::PgPool;
use app_core::prelude::*;

pub async fn create(db: &PgPool, charge: &Charge) -> Result<(), app_error::AppError> {
    sqlx::query(
        r#"INSERT INTO charges(id,patient_id,encounter_id,order_id,code,name,qty,unit_price,amount,currency,status)
           VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)"#
    )
    .bind(charge.id)
    .bind(charge.patient_id)
    .bind(charge.encounter_id)
    .bind(charge.order_id)
    .bind(&charge.code)
    .bind(&charge.name)
    .bind(charge.qty.to_string())
    .bind(charge.unit_price.to_string())
    .bind(charge.amount.to_string())
    .bind(&charge.currency)
    .bind(&charge.status)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn find_by_id(db: &PgPool, id: Uuid) -> Result<Option<Charge>, app_error::AppError> {
    let row = sqlx::query(
        r#"SELECT id,patient_id,encounter_id,order_id,code,name,qty,unit_price,amount,currency,status,created_at,updated_at
           FROM charges WHERE id=$1"#
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| Charge {
        id: r.get("id"),
        patient_id: r.get("patient_id"),
        encounter_id: r.get("encounter_id"),
        order_id: r.get("order_id"),
        code: r.get("code"),
        name: r.get("name"),
        qty: r.get::<String, _>("qty").parse().unwrap_or_default(),
        unit_price: r.get::<String, _>("unit_price").parse().unwrap_or_default(),
        amount: r.get::<String, _>("amount").parse().unwrap_or_default(),
        currency: r.get("currency"),
        status: r.get("status"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}

pub async fn update(
    db: &PgPool,
    id: Uuid,
    name: Option<&str>,
    qty: Option<f64>,
    price: Option<f64>,
    status: Option<&str>,
) -> Result<Option<Charge>, app_error::AppError> {
    let row = sqlx::query(
        r#"UPDATE charges SET
              name=COALESCE($2,name),
              qty=COALESCE($3::NUMERIC,qty),
              unit_price=COALESCE($4::NUMERIC,unit_price),
              status=COALESCE($5,status),
              updated_at=NOW()
           WHERE id=$1
           RETURNING id,patient_id,encounter_id,order_id,code,name,qty,unit_price,amount,currency,status,created_at,updated_at"#
    )
    .bind(id)
    .bind(name)
    .bind(qty.map(|f| f.to_string()))
    .bind(price.map(|f| f.to_string()))
    .bind(status)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| Charge {
        id: r.get("id"),
        patient_id: r.get("patient_id"),
        encounter_id: r.get("encounter_id"),
        order_id: r.get("order_id"),
        code: r.get("code"),
        name: r.get("name"),
        qty: r.get::<String, _>("qty").parse().unwrap_or_default(),
        unit_price: r.get::<String, _>("unit_price").parse().unwrap_or_default(),
        amount: r.get::<String, _>("amount").parse().unwrap_or_default(),
        currency: r.get("currency"),
        status: r.get("status"),
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
) -> Result<(Vec<Charge>, i64), app_error::AppError> {
    let page = page.max(1);
    let size = size.clamp(1, 200);
    let offset = (page - 1) * size;

    let (rows, total) = if let Some(e_id) = encounter_id {
        if let Some(s) = status {
            let r = sqlx::query(
                r#"SELECT id,patient_id,encounter_id,order_id,code,name,qty,unit_price,amount,currency,status,created_at,updated_at
                   FROM charges WHERE encounter_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#
            )
            .bind(e_id)
            .bind(s)
            .bind(offset)
            .bind(size)
            .fetch_all(db)
            .await?;
            let t = sqlx::query(r#"SELECT COUNT(1) FROM charges WHERE encounter_id=$1 AND status=$2"#)
                .bind(e_id)
                .bind(s)
                .fetch_one(db)
                .await?
                .get::<i64, _>(0);
            (r, t)
        } else {
            let r = sqlx::query(
                r#"SELECT id,patient_id,encounter_id,order_id,code,name,qty,unit_price,amount,currency,status,created_at,updated_at
                   FROM charges WHERE encounter_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#
            )
            .bind(e_id)
            .bind(offset)
            .bind(size)
            .fetch_all(db)
            .await?;
            let t = sqlx::query(r#"SELECT COUNT(1) FROM charges WHERE encounter_id=$1"#)
                .bind(e_id)
                .fetch_one(db)
                .await?
                .get::<i64, _>(0);
            (r, t)
        }
    } else {
        let r = sqlx::query(
            r#"SELECT id,patient_id,encounter_id,order_id,code,name,qty,unit_price,amount,currency,status,created_at,updated_at
               FROM charges ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
        )
        .bind(offset)
        .bind(size)
        .fetch_all(db)
        .await?;
        let t = sqlx::query(r#"SELECT COUNT(1) FROM charges"#)
            .fetch_one(db)
            .await?
            .get::<i64, _>(0);
        (r, t)
    };

    let items: Vec<Charge> = rows.into_iter().map(|r| Charge {
        id: r.get("id"),
        patient_id: r.get("patient_id"),
        encounter_id: r.get("encounter_id"),
        order_id: r.get("order_id"),
        code: r.get("code"),
        name: r.get("name"),
        qty: r.get::<String, _>("qty").parse().unwrap_or_default(),
        unit_price: r.get::<String, _>("unit_price").parse().unwrap_or_default(),
        amount: r.get::<String, _>("amount").parse().unwrap_or_default(),
        currency: r.get("currency"),
        status: r.get("status"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect();

    Ok((items, total))
}
