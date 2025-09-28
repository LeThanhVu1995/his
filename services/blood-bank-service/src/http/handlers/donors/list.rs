use actix_web::{get, web, HttpResponse, Responder};
use crate::http::dto::donor_dto::{ListDonorsQuery, ListDonorsResponse};

#[get("/api/v1/blood/donors")]
pub async fn list_donors(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<ListDonorsQuery>,
) -> impl Responder {
    let donor_repo = crate::infra::db::repositories::donor_repo::DonorRepo { db: &db };
    let donor_svc = crate::domain::services::donor_svc::DonorService { donor_repo };

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    match donor_svc.list_donors(limit, offset).await {
        Ok(donors) => {
            HttpResponse::Ok().json(ListDonorsResponse {
                total: donors.len(),
                donors,
                limit,
                offset,
            })
        }
        Err(e) => {
            tracing::error!("Failed to list donors: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to list donors",
                "details": e.to_string()
            }))
        }
    }
}
