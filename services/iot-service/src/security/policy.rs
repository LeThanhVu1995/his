pub mod perm {
    pub const IOT_DEVICE_UPSERT: &str = "his.iot.device.upsert";
    pub const IOT_VITAL_INGEST: &str = "his.iot.vital.ingest";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(IOT_DEVICE_UPSERT, "Upsert device", "iot", "device.upsert"),
        PermissionDef::new(IOT_VITAL_INGEST, "Ingest vitals via HTTP", "iot", "vital.ingest"),
    ]
}


