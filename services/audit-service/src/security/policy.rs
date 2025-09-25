pub mod perm {
    pub const AUDIT_READ: &str = "his.audit.read";
    pub const AUDIT_EXPORT: &str = "his.audit.export";
    pub const AUDIT_WRITE: &str = "his.audit.write";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(AUDIT_READ, "Read audit events", "audit", "read"),
        PermissionDef::new(AUDIT_EXPORT, "Export audit events", "audit", "export"),
        PermissionDef::new(AUDIT_WRITE, "Write audit events", "audit", "write"),
    ]
}


