use actix_web::{get, web, HttpResponse, Responder};
use crate::http::dto::donation_dto::{ListDonationsQuery, ListDonationsResponse};

#[get("/api/v1/blood/donations")]
pub async fn list_donations(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<ListDonationsQuery>,
) -> impl Responder {
    let donation_repo = crate::infra::db::repositories::donation_repo::DonationRepo { db: &db };
    let donation_svc = crate::domain::services::donation_svc::DonationService {
        donation_repo,
        blood_unit_repo: crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db }
    };

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let donations = if let Some(donor_id) = query.donor_id {
        donation_svc.list_donations_by_donor(donor_id, limit, offset).await
    } else {
        donation_svc.list_recent_donations(limit).await
    };

    match donations {
        Ok(donations) => {
            HttpResponse::Ok().json(ListDonationsResponse {
                total: donations.len(),
                donations,
                limit,
                offset,
            })
        }
        Err(e) => {
            tracing::error!("Failed to list donations: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to list donations",
                "details": e.to_string()
            }))
        }
    }
}
