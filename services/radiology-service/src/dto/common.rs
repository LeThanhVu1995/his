use sha2::{Digest, Sha256};

pub fn calc_etag(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("\"{:x}\"", h.finalize())
}
