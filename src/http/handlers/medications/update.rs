use actix_web::{put, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::MedRepo;
use crate::http::dto::medication_dto::{UpdateMedicationReq, MedicationRes};

#[put("/api/v1/medications/{id}")]
pub async fn update_med(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateMedicationReq>,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = MedRepo { db: &db };
    let rec = repo
        .update(
            id,
            payload.name.as_deref(),
            payload.strength.as_deref(),
            payload.form.as_deref(),
            payload.route.as_deref(),
        )
        .await
        .map_err(|e| {
            tracing::error!(?e, "update med");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = MedicationRes {
        id: rec.id,
        code: rec.code,
        name: rec.name,
        strength: rec.strength,
        form: rec.form,
        route: rec.route,
    };
    Ok(HttpResponse::Ok().json(res))
}
