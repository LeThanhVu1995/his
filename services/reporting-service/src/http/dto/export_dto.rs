use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct ExportRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}
