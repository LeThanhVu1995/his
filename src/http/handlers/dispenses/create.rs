use actix_web::{post, web, HttpResponse};
use crate::domain::repo::DispenseRepo;
use crate::domain::service::PharmacyService;
use crate::http::dto::dispense_dto::{CreateDispenseReq, DispenseRes};

#[post("/api/v1/dispenses:create")]
pub async fn create_dispense(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateDispenseReq>,
) -> actix_web::Result<HttpResponse> {
    let svc = PharmacyService {
        meds: crate::domain::repo::MedRepo { db: &db },
        presc: crate::domain::repo::PrescRepo { db: &db },
        items: crate::domain::repo::PrescItemRepo { db: &db },
        disp: DispenseRepo { db: &db },
    };
    let id = svc
        .create_dispense(payload.prescription_id, None)
        .await
        .map_err(|e| {
            tracing::error!(?e, "create disp");
            crate::error::AppError::Internal("DB".into())
        })?;
    let repo = DispenseRepo { db: &db };
    let d = sqlx::query_as!(
        crate::domain::models::Dispense,
        r#"SELECT id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at FROM dispenses WHERE id=$1"#,
        id
    )
    .fetch_optional(&**db)
    .await
    .map_err(|_| crate::error::AppError::Internal("DB".into()))?
    .ok_or(crate::error::AppError::NotFound)?;
    let res = DispenseRes {
        id: d.id,
        disp_no: d.disp_no,
        status: d.status,
    };
    Ok(HttpResponse::Created().json(res))
}
