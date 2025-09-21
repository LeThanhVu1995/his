use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::domain::repositories::PrescriptionRepo;
use crate::http::dto::prescription_dto::{UpdatePrescriptionReq, PrescriptionRes};


#[utoipa::path(
    put,
    path = "/api/v1/prescriptions/{id}",
    params(("id" = Uuid, Path, description = "Prescription ID")),
    request_body = UpdatePrescriptionReq,
    responses(
        (status = 200, description = "Prescription updated successfully", body = PrescriptionRes),
        (status = 404, description = "Prescription not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_prescription(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: Json<UpdatePrescriptionReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = PrescriptionRepo { db: &db };
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
        patient_id: rec.patient_id,
        encounter_id: rec.encounter_id,
        presc_no: rec.presc_no,
        status: rec.status,
        ordered_by: rec.ordered_by,
        note: rec.note,
    };

    Ok(HttpResponse::Ok().json(res))
}
