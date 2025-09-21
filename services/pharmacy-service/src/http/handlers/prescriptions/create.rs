use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use validator::Validate;

use app_web::prelude::AuthUser;
use crate::domain::service::PharmacyService;
use crate::http::dto::prescription_dto::{CreatePrescriptionReq, PrescriptionRes};

pub async fn create_prescription(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreatePrescriptionReq>,
    user: AuthUser,
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

    let ordered_by = Some(user.subject.as_str());
    let id = svc
        .create_prescription(&payload.into_inner(), ordered_by)
        .await
        .map_err(|e| {
            tracing::error!(?e, "create presc");
            crate::error::AppError::Internal("DB".into())
        })?;

    let repo = crate::domain::repositories::PrescriptionRepo { db: &db };
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
        patient_id: p.patient_id,
        encounter_id: p.encounter_id,
        presc_no: p.presc_no,
        status: p.status,
        ordered_by: p.ordered_by,
        note: p.note,
    };

    Ok(HttpResponse::Created().json(res))
}
