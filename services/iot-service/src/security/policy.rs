pub mod perm {
    // Device Management
    pub const IOT_DEVICE_UPSERT: &str = "his.iot.device.upsert";
    pub const IOT_DEVICE_GET: &str = "his.iot.device.get";
    pub const IOT_DEVICE_LIST: &str = "his.iot.device.list";
    pub const IOT_DEVICE_UPDATE: &str = "his.iot.device.update";
    pub const IOT_DEVICE_DELETE: &str = "his.iot.device.delete";

    // Vital Signs Management
    pub const IOT_VITAL_CREATE: &str = "his.iot.vital.create";
    pub const IOT_VITAL_GET: &str = "his.iot.vital.get";
    pub const IOT_VITAL_LIST: &str = "his.iot.vital.list";
    pub const IOT_VITAL_UPDATE: &str = "his.iot.vital.update";
    pub const IOT_VITAL_DELETE: &str = "his.iot.vital.delete";
    pub const IOT_VITAL_INGEST: &str = "his.iot.vital.ingest";

    // Observations Management
    pub const IOT_OBSERVATION_CREATE: &str = "his.iot.observation.create";
    pub const IOT_OBSERVATION_GET: &str = "his.iot.observation.get";
    pub const IOT_OBSERVATION_LIST: &str = "his.iot.observation.list";
    pub const IOT_OBSERVATION_UPDATE: &str = "his.iot.observation.update";
    pub const IOT_OBSERVATION_DELETE: &str = "his.iot.observation.delete";

    // Device Readings Management
    pub const IOT_READING_CREATE: &str = "his.iot.reading.create";
    pub const IOT_READING_GET: &str = "his.iot.reading.get";
    pub const IOT_READING_LIST: &str = "his.iot.reading.list";
    pub const IOT_READING_UPDATE: &str = "his.iot.reading.update";
    pub const IOT_READING_DELETE: &str = "his.iot.reading.delete";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        // Device Management
        PermissionDef::new(IOT_DEVICE_UPSERT, "Upsert device", "iot", "device.upsert"),
        PermissionDef::new(IOT_DEVICE_GET, "Get device", "iot", "device.get"),
        PermissionDef::new(IOT_DEVICE_LIST, "List devices", "iot", "device.list"),
        PermissionDef::new(IOT_DEVICE_UPDATE, "Update device", "iot", "device.update"),
        PermissionDef::new(IOT_DEVICE_DELETE, "Delete device", "iot", "device.delete"),

        // Vital Signs Management
        PermissionDef::new(IOT_VITAL_CREATE, "Create vital signs", "iot", "vital.create"),
        PermissionDef::new(IOT_VITAL_GET, "Get vital signs", "iot", "vital.get"),
        PermissionDef::new(IOT_VITAL_LIST, "List vital signs", "iot", "vital.list"),
        PermissionDef::new(IOT_VITAL_UPDATE, "Update vital signs", "iot", "vital.update"),
        PermissionDef::new(IOT_VITAL_DELETE, "Delete vital signs", "iot", "vital.delete"),
        PermissionDef::new(IOT_VITAL_INGEST, "Ingest vitals via HTTP", "iot", "vital.ingest"),

        // Observations Management
        PermissionDef::new(IOT_OBSERVATION_CREATE, "Create observation", "iot", "observation.create"),
        PermissionDef::new(IOT_OBSERVATION_GET, "Get observation", "iot", "observation.get"),
        PermissionDef::new(IOT_OBSERVATION_LIST, "List observations", "iot", "observation.list"),
        PermissionDef::new(IOT_OBSERVATION_UPDATE, "Update observation", "iot", "observation.update"),
        PermissionDef::new(IOT_OBSERVATION_DELETE, "Delete observation", "iot", "observation.delete"),

        // Device Readings Management
        PermissionDef::new(IOT_READING_CREATE, "Create device reading", "iot", "reading.create"),
        PermissionDef::new(IOT_READING_GET, "Get device reading", "iot", "reading.get"),
        PermissionDef::new(IOT_READING_LIST, "List device readings", "iot", "reading.list"),
        PermissionDef::new(IOT_READING_UPDATE, "Update device reading", "iot", "reading.update"),
        PermissionDef::new(IOT_READING_DELETE, "Delete device reading", "iot", "reading.delete"),
    ]
}
