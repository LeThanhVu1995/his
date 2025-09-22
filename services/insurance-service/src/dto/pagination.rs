use serde::Deserialize;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct Page {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
