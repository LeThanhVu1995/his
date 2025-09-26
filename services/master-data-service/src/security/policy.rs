/// Chuẩn hoá: his.master.<entity>.<action>
/// entity: code, department, specialty, country, ...
/// action: read, create, update, delete, list, export...

pub mod perm {
    pub const MASTER_CODE_LIST: &str   = "his.master.code.list";
    pub const MASTER_CODE_READ: &str   = "his.master.code.read";
    pub const MASTER_CODE_CREATE: &str = "his.master.code.create";
    pub const MASTER_CODE_UPDATE: &str = "his.master.code.update";
    pub const MASTER_CODE_DELETE: &str = "his.master.code.delete";
}

/// Gợi ý role mẫu trong IAM (do IAM quản lý):
/// - ROLE_MASTER_ADMIN  -> * (tất cả his.master.*)
/// - ROLE_MASTER_EDITOR -> create/update/delete + list/read cho code
/// - ROLE_MASTER_VIEWER -> list/read

pub fn permission_catalog(service_name: &str) -> Vec<app_web::security::PermissionDef> {
    use perm::*;
    vec![
        app_web::security::PermissionDef::new(MASTER_CODE_LIST,   "List codes", service_name, "list"),
        app_web::security::PermissionDef::new(MASTER_CODE_READ,   "Read code",  service_name, "read"),
        app_web::security::PermissionDef::new(MASTER_CODE_CREATE, "Create code",service_name, "create"),
        app_web::security::PermissionDef::new(MASTER_CODE_UPDATE, "Update code",service_name, "update"),
        app_web::security::PermissionDef::new(MASTER_CODE_DELETE, "Delete code",service_name, "delete"),
    ]
}
