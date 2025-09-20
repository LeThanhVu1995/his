use actix_web::{put, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::PrescRepo;
use crate::http::dto::prescription_dto::{UpdatePrescriptionReq, PrescriptionRes};

#[put("/api/v1/prescriptions/{id}")]
pub async fn update_prescription(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdatePrescriptionReq>,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = PrescRepo { db: &db };
    let rec = repo
        .update(id, payload.status.as_deref(), payload.note.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(?e, "update presc");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = PrescriptionRes {
        id: rec.id,
        presc_no: rec.presc_no,
        status: rec.status,
    };
    Ok(HttpResponse::Ok().json(res))
}
