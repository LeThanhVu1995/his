use actix_web::{get, web, HttpResponse, Responder};
use crate::http::dto::blood_unit_dto::{SearchUnitsQuery, SearchUnitsResponse};

#[get("/api/v1/blood/units")]
pub async fn search_units(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<SearchUnitsQuery>,
) -> impl Responder {
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let blood_unit_svc = crate::domain::services::blood_unit_svc::BloodUnitService { blood_unit_repo };

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    match blood_unit_svc.get_available_units(
        query.blood_group.as_deref(),
        query.component_code.as_deref(),
    ).await {
        Ok(units) => {
            HttpResponse::Ok().json(SearchUnitsResponse {
                total: units.len(),
                units,
                limit,
                offset,
            })
        }
        Err(e) => {
            tracing::error!("Failed to search blood units: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to search blood units",
                "details": e.to_string()
            }))
        }
    }
}
