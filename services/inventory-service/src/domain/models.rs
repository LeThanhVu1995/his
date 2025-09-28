use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Warehouse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub r#type: String,
    pub facility_id: Option<Uuid>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Item {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub uom: String,
    pub base_uom_id: Option<Uuid>,
    pub category_code: Option<String>,
    pub is_med: bool,
    pub is_consumable: bool,
    pub is_lot_tracked: bool,
    pub is_expirable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Lot {
    pub id: Uuid,
    pub item_id: Uuid,
    pub lot_no: String,
    pub exp_date: Option<NaiveDate>,
    pub supplier_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Stock {
    pub warehouse_id: Uuid,
    pub item_id: Uuid,
    pub lot_id: Option<Uuid>,
    pub qty: Decimal,
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
    pub qty: Decimal,
}

// New models for enhanced inventory management

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Uom {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ItemUom {
    pub id: Uuid,
    pub item_id: Uuid,
    pub uom_id: Uuid,
    pub factor: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Supplier {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub tax_id: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub supplier_id: Uuid,
    pub facility_id: Uuid,
    pub po_no: String,
    pub status: String,
    pub ordered_at: DateTime<Utc>,
    pub expected_delivery_date: Option<NaiveDate>,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PurchaseOrderItem {
    pub id: Uuid,
    pub po_id: Uuid,
    pub item_id: Uuid,
    pub quantity: Decimal,
    pub uom_id: Uuid,
    pub unit_price: Decimal,
    pub total_price: Decimal,
    pub received_quantity: Decimal,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct GoodsReceipt {
    pub id: Uuid,
    pub po_id: Option<Uuid>,
    pub warehouse_id: Uuid,
    pub grn_no: String,
    pub received_at: DateTime<Utc>,
    pub received_by: Option<String>,
    pub status: String,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct GoodsReceiptItem {
    pub id: Uuid,
    pub grn_id: Uuid,
    pub item_id: Uuid,
    pub batch_id: Option<Uuid>,
    pub quantity: Decimal,
    pub uom_id: Uuid,
    pub unit_price: Option<Decimal>,
    pub total_price: Option<Decimal>,
    pub expiry_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct StockTransaction {
    pub id: Uuid,
    pub warehouse_id: Uuid,
    pub item_id: Uuid,
    pub batch_id: Option<Uuid>,
    pub qty_delta: Decimal,
    pub uom_id: Uuid,
    pub reason_code: String,
    pub ref_entity: Option<String>,
    pub ref_id: Option<Uuid>,
    pub occurred_at: DateTime<Utc>,
    pub performed_by: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
