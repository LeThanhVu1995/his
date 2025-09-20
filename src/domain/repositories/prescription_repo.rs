use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use crate::domain::entities::prescription::Prescription;
use crate::error::AppError;

pub async fn create(db: &Pool<Postgres>, prescription: &Prescription) -> Result<(), AppError> {
    sqlx::query!(
        r#"INSERT INTO prescriptions(id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)"#,
        prescription.id,
        prescription.patient_id,
        prescription.encounter_id,
        prescription.presc_no,
        prescription.status,
        prescription.ordered_by,
        prescription.note,
        prescription.created_at,
        prescription.updated_at
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "create prescription");
        AppError::Internal("DB".into())
    })?;
    Ok(())
}

pub async fn update(
    db: &Pool<Postgres>,
    id: Uuid,
    status: Option<&str>,
    note: Option<&str>,
) -> Result<Option<Prescription>, AppError> {
    let rec = sqlx::query!(
        r#"UPDATE prescriptions SET status=COALESCE($2,status), note=COALESCE($3,note), updated_at=NOW() WHERE id=$1 RETURNING id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at"#,
        id, status, note
    )
    .fetch_optional(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "update prescription");
        AppError::Internal("DB".into())
    })?;

    if let Some(r) = rec {
        Ok(Some(Prescription {
            id: r.id,
            patient_id: r.patient_id,
            encounter_id: r.encounter_id,
            presc_no: r.presc_no,
            status: r.status,
            ordered_by: r.ordered_by,
            note: r.note,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    } else {
        Ok(None)
    }
}

pub async fn list_paged(
    db: &Pool<Postgres>,
    patient_id: Option<Uuid>,
    status: Option<&str>,
    page: i64,
    size: i64,
) -> Result<(Vec<Prescription>, i64), AppError> {
    let page = page.max(1);
    let size = size.clamp(1, 200);
    let offset = (page - 1) * size;

    let (rows, total) = match (patient_id, status) {
        (Some(p), Some(s)) => {
            let r = sqlx::query!(
                r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions WHERE patient_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#,
                p, s, offset, size
            )
            .fetch_all(db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "list prescriptions");
                AppError::Internal("DB".into())
            })?;

            let t = sqlx::query_scalar!(
                "SELECT COUNT(1) FROM prescriptions WHERE patient_id=$1 AND status=$2",
                p, s
            )
            .fetch_one(db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "count prescriptions");
                AppError::Internal("DB".into())
            })?;

            let prescriptions: Vec<Prescription> = r.into_iter().map(|r| Prescription {
                id: r.id,
                patient_id: r.patient_id,
                encounter_id: r.encounter_id,
                presc_no: r.presc_no,
                status: r.status,
                ordered_by: r.ordered_by,
                note: r.note,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }).collect();

            (prescriptions, t)
        }
        (Some(p), None) => {
            let r = sqlx::query!(
                r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions WHERE patient_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#,
                p, offset, size
            )
            .fetch_all(db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "list prescriptions");
                AppError::Internal("DB".into())
            })?;

            let t = sqlx::query_scalar!(
                "SELECT COUNT(1) FROM prescriptions WHERE patient_id=$1",
                p
            )
            .fetch_one(db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "count prescriptions");
                AppError::Internal("DB".into())
            })?;

            let prescriptions: Vec<Prescription> = r.into_iter().map(|r| Prescription {
                id: r.id,
                patient_id: r.patient_id,
                encounter_id: r.encounter_id,
                presc_no: r.presc_no,
                status: r.status,
                ordered_by: r.ordered_by,
                note: r.note,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }).collect();

            (prescriptions, t)
        }
        _ => {
            let r = sqlx::query!(
                r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions ORDER BY created_at DESC OFFSET $1 LIMIT $2"#,
                offset, size
            )
            .fetch_all(db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "list prescriptions");
                AppError::Internal("DB".into())
            })?;

            let t = sqlx::query_scalar!("SELECT COUNT(1) FROM prescriptions")
                .fetch_one(db)
                .await
                .map_err(|e| {
                    tracing::error!(?e, "count prescriptions");
                    AppError::Internal("DB".into())
                })?;

            let prescriptions: Vec<Prescription> = r.into_iter().map(|r| Prescription {
                id: r.id,
                patient_id: r.patient_id,
                encounter_id: r.encounter_id,
                presc_no: r.presc_no,
                status: r.status,
                ordered_by: r.ordered_by,
                note: r.note,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }).collect();

            (prescriptions, t)
        }
    };

    Ok((rows, total))
}

pub async fn find(db: &Pool<Postgres>, id: Uuid) -> Result<Option<Prescription>, AppError> {
    let rec = sqlx::query!(
        r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions WHERE id=$1"#,
        id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "find prescription");
        AppError::Internal("DB".into())
    })?;

    if let Some(r) = rec {
        Ok(Some(Prescription {
            id: r.id,
            patient_id: r.patient_id,
            encounter_id: r.encounter_id,
            presc_no: r.presc_no,
            status: r.status,
            ordered_by: r.ordered_by,
            note: r.note,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    } else {
        Ok(None)
    }
}
