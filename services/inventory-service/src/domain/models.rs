use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
// use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Warehouse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub r#type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Item {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub uom: String,
    pub is_med: bool,
    pub is_consumable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Lot {
    pub id: Uuid,
    pub item_id: Uuid,
    pub lot_no: String,
    pub exp_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Stock {
    pub warehouse_id: Uuid,
    pub item_id: Uuid,
    pub lot_id: Option<Uuid>,
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Movement {
    pub id: Uuid,
    pub mv_no: String,
    pub mv_type: String,
    pub src_wh: Option<Uuid>,
    pub dst_wh: Option<Uuid>,
    pub note: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct MovementLine {
    pub id: Uuid,
    pub movement_id: Uuid,
    pub item_id: Uuid,
    pub lot_id: Option<Uuid>,
    pub qty: f64,
}
