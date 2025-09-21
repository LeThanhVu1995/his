use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::domain::repositories::MedRepo;
use crate::http::dto::medication_dto::{UpdateMedicationReq, MedicationRes};

pub async fn update_med(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: Json<UpdateMedicationReq>,
    _user: AuthUser,
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
