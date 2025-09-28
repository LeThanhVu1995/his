use sha2::{Digest, Sha256};
use validator::ValidationError;

pub fn calc_etag(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("\"{:x}\"", h.finalize())
}

pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
    if uuid::Uuid::parse_str(uuid).is_ok() {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_uuid"))
    }
}

pub fn validate_uuid_option(uuid: &Option<String>) -> Result<(), ValidationError> {
    match uuid {
        Some(u) => validate_uuid(u),
        None => Ok(()),
    }
}
