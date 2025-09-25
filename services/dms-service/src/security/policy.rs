pub mod perm {
    pub const DMS_OBJECT_UPLOAD: &str = "his.dms.object.upload";
    pub const DMS_OBJECT_DOWNLOAD: &str = "his.dms.object.download";
    pub const DMS_SIGNATURE_ATTACH: &str = "his.dms.signature.attach";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(DMS_OBJECT_UPLOAD, "Presign upload", "dms", "object.upload"),
        PermissionDef::new(DMS_OBJECT_DOWNLOAD, "Presign download", "dms", "object.download"),
        PermissionDef::new(DMS_SIGNATURE_ATTACH, "Attach digital signature", "dms", "signature.attach"),
    ]
}


