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

#[derive(serde::Serialize)]
pub struct PermissionDef {
    pub name: String,      // ví dụ: his.master.code.read
    pub description: String,
    pub service: String,   // master-data-service
}

pub fn permission_catalog(service_name: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef { name: MASTER_CODE_LIST.into(),   description: "List codes".into(),   service: service_name.into() },
        PermissionDef { name: MASTER_CODE_READ.into(),   description: "Read code".into(),    service: service_name.into() },
        PermissionDef { name: MASTER_CODE_CREATE.into(), description: "Create code".into(),  service: service_name.into() },
        PermissionDef { name: MASTER_CODE_UPDATE.into(), description: "Update code".into(),  service: service_name.into() },
        PermissionDef { name: MASTER_CODE_DELETE.into(), description: "Delete code".into(),  service: service_name.into() },
    ]
}
