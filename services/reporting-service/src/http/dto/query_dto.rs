use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct RangeQuery {
    pub start: NaiveDate,
    pub end: NaiveDate,
}
