use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use crate::domain::entities::prescription_item::PrescriptionItem;
use crate::error::AppError;

pub async fn insert_many(db: &Pool<Postgres>, items: &[PrescriptionItem]) -> Result<(), AppError> {
    for item in items {
        sqlx::query!(
            r#"INSERT INTO prescription_items(id,prescription_id,medication_id,dose,freq,duration,qty,instruction,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#,
            item.id,
            item.prescription_id,
            item.medication_id,
            item.dose,
            item.freq,
            item.duration,
            item.qty.to_string(),
            item.instruction,
            item.created_at,
            item.updated_at
        )
        .execute(db)
        .await
        .map_err(|e| {
            tracing::error!(?e, "insert prescription item");
            AppError::Internal("DB".into())
        })?;
    }
    Ok(())
}

pub async fn list_by_prescription(db: &Pool<Postgres>, prescription_id: Uuid) -> Result<Vec<PrescriptionItem>, AppError> {
    let rows = sqlx::query!(
        r#"SELECT id,prescription_id,medication_id,dose,freq,duration,qty,instruction,created_at,updated_at FROM prescription_items WHERE prescription_id=$1 ORDER BY created_at"#,
        prescription_id
    )
    .fetch_all(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "list prescription items");
        AppError::Internal("DB".into())
    })?;

    let items: Vec<PrescriptionItem> = rows.into_iter().map(|r| PrescriptionItem {
        id: r.id,
        prescription_id: r.prescription_id,
        medication_id: r.medication_id,
        dose: r.dose,
        freq: r.freq,
        duration: r.duration,
        qty: r.qty.parse().unwrap_or_default(),
        instruction: r.instruction,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect();

    Ok(items)
}
