use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use crate::domain::entities::dispense::Dispense;
use crate::error::AppError;

pub async fn create(db: &Pool<Postgres>, dispense: &Dispense) -> Result<(), AppError> {
    sqlx::query!(
        r#"INSERT INTO dispenses(id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7)"#,
        dispense.id,
        dispense.prescription_id,
        dispense.disp_no,
        dispense.dispensed_by,
        dispense.status,
        dispense.created_at,
        dispense.updated_at
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "create dispense");
        AppError::Internal("DB".into())
    })?;
    Ok(())
}

pub async fn finish(db: &Pool<Postgres>, id: Uuid) -> Result<Option<Dispense>, AppError> {
    let rec = sqlx::query!(
        r#"UPDATE dispenses SET status='COMPLETED', updated_at=NOW() WHERE id=$1 RETURNING id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at"#,
        id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "finish dispense");
        AppError::Internal("DB".into())
    })?;

    if let Some(r) = rec {
        Ok(Some(Dispense {
            id: r.id,
            prescription_id: r.prescription_id,
            disp_no: r.disp_no,
            dispensed_by: r.dispensed_by,
            status: r.status,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    } else {
        Ok(None)
    }
}

pub async fn list_paged(
    db: &Pool<Postgres>,
    prescription_id: Option<Uuid>,
    page: i64,
    size: i64,
) -> Result<(Vec<Dispense>, i64), AppError> {
    let page = page.max(1);
    let size = size.clamp(1, 200);
    let offset = (page - 1) * size;

    let (rows, total) = match prescription_id {
        Some(pid) => {
            let r = sqlx::query!(
                r#"SELECT id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at FROM dispenses WHERE prescription_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#,
                pid, offset, size
            )
            .fetch_all(db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "list dispenses");
                AppError::Internal("DB".into())
            })?;

            let t = sqlx::query_scalar!(
                "SELECT COUNT(1) FROM dispenses WHERE prescription_id=$1",
                pid
            )
            .fetch_one(db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "count dispenses");
                AppError::Internal("DB".into())
            })?;

            let dispenses: Vec<Dispense> = r.into_iter().map(|r| Dispense {
                id: r.id,
                prescription_id: r.prescription_id,
                disp_no: r.disp_no,
                dispensed_by: r.dispensed_by,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }).collect();

            (dispenses, t)
        }
        None => {
            let r = sqlx::query!(
                r#"SELECT id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at FROM dispenses ORDER BY created_at DESC OFFSET $1 LIMIT $2"#,
                offset, size
            )
            .fetch_all(db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "list dispenses");
                AppError::Internal("DB".into())
            })?;

            let t = sqlx::query_scalar!("SELECT COUNT(1) FROM dispenses")
                .fetch_one(db)
                .await
                .map_err(|e| {
                    tracing::error!(?e, "count dispenses");
                    AppError::Internal("DB".into())
                })?;

            let dispenses: Vec<Dispense> = r.into_iter().map(|r| Dispense {
                id: r.id,
                prescription_id: r.prescription_id,
                disp_no: r.disp_no,
                dispensed_by: r.dispensed_by,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }).collect();

            (dispenses, t)
        }
    };

    Ok((rows, total))
}

pub async fn find(db: &Pool<Postgres>, id: Uuid) -> Result<Option<Dispense>, AppError> {
    let rec = sqlx::query!(
        r#"SELECT id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at FROM dispenses WHERE id=$1"#,
        id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "find dispense");
        AppError::Internal("DB".into())
    })?;

    if let Some(r) = rec {
        Ok(Some(Dispense {
            id: r.id,
            prescription_id: r.prescription_id,
            disp_no: r.disp_no,
            dispensed_by: r.dispensed_by,
            status: r.status,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    } else {
        Ok(None)
    }
}
