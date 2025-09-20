use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calc_etag<T: Hash>(data: &T) -> String {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("\"{:x}\"", hasher.finish())
}
