use actix_web::{post, web, HttpResponse};
use crate::domain::repo::MedRepo;
use crate::domain::service::PharmacyService;
use crate::http::dto::medication_dto::{CreateMedicationReq, MedicationRes};

#[post("/api/v1/medications:create")]
pub async fn create_med(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateMedicationReq>,
) -> actix_web::Result<HttpResponse> {
    if let Err(e) = payload.validate() {
        return Err(crate::error::AppError::BadRequest(e.to_string()).into());
    }
    let svc = PharmacyService {
        meds: MedRepo { db: &db },
        presc: crate::domain::repo::PrescRepo { db: &db },
        items: crate::domain::repo::PrescItemRepo { db: &db },
        disp: crate::domain::repo::DispenseRepo { db: &db },
    };
    let id = svc
        .create_med(&payload)
        .await
        .map_err(|e| {
            tracing::error!(?e, "create med");
            crate::error::AppError::Internal("DB".into())
        })?;
    let repo = MedRepo { db: &db };
    let m = repo
        .find(id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "find med");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = MedicationRes {
        id: m.id,
        code: m.code,
        name: m.name,
        strength: m.strength,
        form: m.form,
        route: m.route,
    };
    Ok(HttpResponse::Created().json(res))
}
