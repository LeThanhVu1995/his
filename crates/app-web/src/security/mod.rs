// src/security/mod.rs - Security utilities
use serde::{Serialize, Deserialize};

/// Generic permission definition that can be serialized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDef {
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
}

impl PermissionDef {
    pub fn new(name: &str, description: &str, resource: &str, action: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            resource: resource.to_string(),
            action: action.to_string(),
        }
    }
}

/// Trait for services to define their permission catalog
pub trait PermissionCatalog {
    fn get_permissions(service_name: &str) -> Vec<PermissionDef>;
}
