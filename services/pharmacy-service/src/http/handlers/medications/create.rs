use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use validator::Validate;
use app_web::prelude::AuthUser;
use crate::domain::service::PharmacyService;
use crate::http::dto::medication_dto::{CreateMedicationReq, MedicationRes};

pub async fn create_med(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateMedicationReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    if let Err(e) = payload.validate() {
        return Err(crate::error::AppError::BadRequest(e.to_string()).into());
    }

    let svc = PharmacyService {
        meds: crate::domain::repositories::MedRepo { db: &db },
        presc: crate::domain::repositories::PrescriptionRepo { db: &db },
        items: crate::domain::repositories::PrescriptionItemRepo { db: &db },
        disp: crate::domain::repositories::DispenseRepo { db: &db },
    };

    let id = svc
        .create_med(&payload.into_inner())
        .await
        .map_err(|e| {
            tracing::error!(?e, "create med");
            crate::error::AppError::Internal("DB".into())
        })?;

    let repo = crate::domain::repositories::MedRepo { db: &db };
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
