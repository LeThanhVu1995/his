use actix_web::{post, web, HttpResponse};
use crate::domain::repo::{PrescRepo, PrescItemRepo};
use crate::domain::service::PharmacyService;
use crate::http::dto::prescription_dto::{CreatePrescriptionReq, PrescriptionRes};
use crate::security::auth_user::AuthUser;

#[post("/api/v1/prescriptions:create")]
pub async fn create_prescription(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreatePrescriptionReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    if let Err(e) = payload.validate() {
        return Err(crate::error::AppError::BadRequest(e.to_string()).into());
    }
    let svc = PharmacyService {
        meds: crate::domain::repo::MedRepo { db: &db },
        presc: PrescRepo { db: &db },
        items: PrescItemRepo { db: &db },
        disp: crate::domain::repo::DispenseRepo { db: &db },
    };
    let ordered_by = Some(user.0.sub.as_str());
    let id = svc
        .create_prescription(&payload, ordered_by)
        .await
        .map_err(|e| {
            tracing::error!(?e, "create presc");
            crate::error::AppError::Internal("DB".into())
        })?;
    let repo = PrescRepo { db: &db };
    let p = repo
        .find(id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "find presc");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = PrescriptionRes {
        id: p.id,
        presc_no: p.presc_no,
        status: p.status,
    };
    Ok(HttpResponse::Created().json(res))
}
