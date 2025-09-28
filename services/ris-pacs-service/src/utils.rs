use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Generate a new UUID
pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

/// Get current timestamp
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Generate DICOM Study UID
pub fn generate_study_uid() -> String {
    format!("1.2.826.0.1.3680043.8.498.{}", Uuid::new_v4())
}

/// Generate DICOM Series UID
pub fn generate_series_uid() -> String {
    format!("1.2.826.0.1.3680043.8.498.{}", Uuid::new_v4())
}

/// Generate DICOM SOP Instance UID
pub fn generate_sop_instance_uid() -> String {
    format!("1.2.826.0.1.3680043.8.498.{}", Uuid::new_v4())
}

/// Generate accession number
pub fn generate_accession_number() -> String {
    let now = Utc::now();
    format!("ACC{}{:06}", now.format("%Y%m%d"), now.timestamp() % 1000000)
}

/// Generate report number
pub fn generate_report_number() -> String {
    let now = Utc::now();
    format!("RPT{}{:06}", now.format("%Y%m%d"), now.timestamp() % 1000000)
}