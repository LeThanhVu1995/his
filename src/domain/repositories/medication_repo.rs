use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use crate::domain::entities::medication::Medication;
use crate::error::AppError;

pub async fn create(db: &Pool<Postgres>, medication: &Medication) -> Result<(), AppError> {
    sqlx::query!(
        r#"INSERT INTO medications(id,code,name,strength,form,route,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8)"#,
        medication.id,
        medication.code,
        medication.name,
        medication.strength,
        medication.form,
        medication.route,
        medication.created_at,
        medication.updated_at
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "create medication");
        AppError::Internal("DB".into())
    })?;
    Ok(())
}

pub async fn update(
    db: &Pool<Postgres>,
    id: Uuid,
    name: Option<&str>,
    strength: Option<&str>,
    form: Option<&str>,
    route: Option<&str>,
) -> Result<Option<Medication>, AppError> {
    let rec = sqlx::query!(
        r#"UPDATE medications SET name=COALESCE($2,name), strength=COALESCE($3,strength), form=COALESCE($4,form), route=COALESCE($5,route), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,strength,form,route,created_at,updated_at"#,
        id, name, strength, form, route
    )
    .fetch_optional(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "update medication");
        AppError::Internal("DB".into())
    })?;

    if let Some(r) = rec {
        Ok(Some(Medication {
            id: r.id,
            code: r.code,
            name: r.name,
            strength: r.strength,
            form: r.form,
            route: r.route,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    } else {
        Ok(None)
    }
}

pub async fn search_paged(
    db: &Pool<Postgres>,
    q: Option<&str>,
    page: i64,
    size: i64,
) -> Result<(Vec<Medication>, i64), AppError> {
    let page = page.max(1);
    let size = size.clamp(1, 200);
    let offset = (page - 1) * size;

    let (rows, total) = if let Some(q) = q {
        let like = format!("%{}%", q);
        let r = sqlx::query!(
            r#"SELECT id,code,name,strength,form,route,created_at,updated_at FROM medications WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY name OFFSET $2 LIMIT $3"#,
            like, offset, size
        )
        .fetch_all(db)
        .await
        .map_err(|e| {
            tracing::error!(?e, "search medications");
            AppError::Internal("DB".into())
        })?;

        let t = sqlx::query_scalar!(
            "SELECT COUNT(1) FROM medications WHERE code ILIKE $1 OR name ILIKE $1",
            like
        )
        .fetch_one(db)
        .await
        .map_err(|e| {
            tracing::error!(?e, "count medications");
            AppError::Internal("DB".into())
        })?;

        let medications: Vec<Medication> = r.into_iter().map(|r| Medication {
            id: r.id,
            code: r.code,
            name: r.name,
            strength: r.strength,
            form: r.form,
            route: r.route,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }).collect();

        (medications, t)
    } else {
        let r = sqlx::query!(
            r#"SELECT id,code,name,strength,form,route,created_at,updated_at FROM medications ORDER BY name OFFSET $1 LIMIT $2"#,
            offset, size
        )
        .fetch_all(db)
        .await
        .map_err(|e| {
            tracing::error!(?e, "list medications");
            AppError::Internal("DB".into())
        })?;

        let t = sqlx::query_scalar!("SELECT COUNT(1) FROM medications")
            .fetch_one(db)
            .await
            .map_err(|e| {
                tracing::error!(?e, "count medications");
                AppError::Internal("DB".into())
            })?;

        let medications: Vec<Medication> = r.into_iter().map(|r| Medication {
            id: r.id,
            code: r.code,
            name: r.name,
            strength: r.strength,
            form: r.form,
            route: r.route,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }).collect();

        (medications, t)
    };

    Ok((rows, total))
}

pub async fn find(db: &Pool<Postgres>, id: Uuid) -> Result<Option<Medication>, AppError> {
    let rec = sqlx::query!(
        r#"SELECT id,code,name,strength,form,route,created_at,updated_at FROM medications WHERE id=$1"#,
        id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| {
        tracing::error!(?e, "find medication");
        AppError::Internal("DB".into())
    })?;

    if let Some(r) = rec {
        Ok(Some(Medication {
            id: r.id,
            code: r.code,
            name: r.name,
            strength: r.strength,
            form: r.form,
            route: r.route,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    } else {
        Ok(None)
    }
}
