use actix_web::{web, HttpResponse};
use crate::infrastructure::repositories::member_repo::MemberRepo;
use crate::domain::services::eligibility_svc::EligibilitySvc;

pub async fn check(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<crate::dto::eligibility_dto::EligibilityReqDto>,
) -> actix_web::Result<HttpResponse> {
    let svc = EligibilitySvc {
        repo: MemberRepo { db: &db },
    };

    let m = svc.check_and_upsert(body.patient_id, &body.payer, &body.policy_no)
        .await
        .map_err(|e| {
            tracing::warn!(?e, "eligibility");
            crate::error::AppError::BadRequest("not eligible".into())
        })?;

    Ok(HttpResponse::Ok().json(crate::dto::eligibility_dto::EligibilityResDto {
        eligible: true,
        member_id: m.id,
        plan_code: m.plan_code,
    }))
}
