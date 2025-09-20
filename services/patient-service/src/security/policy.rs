pub mod perm {
    // Patient
    pub const PATIENT_LIST: &str   = "his.patient.list";
    pub const PATIENT_READ: &str   = "his.patient.read";
    pub const PATIENT_CREATE: &str = "his.patient.create";
    pub const PATIENT_UPDATE: &str = "his.patient.update";
    pub const PATIENT_DELETE: &str = "his.patient.delete";
    // Encounter
    pub const ENCOUNTER_LIST: &str   = "his.encounter.list";
    pub const ENCOUNTER_READ: &str   = "his.encounter.read";
    pub const ENCOUNTER_CREATE: &str = "his.encounter.create";
    pub const ENCOUNTER_UPDATE: &str = "his.encounter.update";
    pub const ENCOUNTER_CLOSE: &str  = "his.encounter.close"; // discharge
}

#[derive(serde::Serialize)]
pub struct PermissionDef {
    pub name: String,
    pub description: String,
    pub service: String
}

pub fn permission_catalog(service: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        // patient
        PermissionDef { name: PATIENT_LIST.into(),   description: "List patients".into(),   service: service.into() },
        PermissionDef { name: PATIENT_READ.into(),   description: "Read patient".into(),   service: service.into() },
        PermissionDef { name: PATIENT_CREATE.into(), description: "Create patient".into(), service: service.into() },
        PermissionDef { name: PATIENT_UPDATE.into(), description: "Update patient".into(), service: service.into() },
        PermissionDef { name: PATIENT_DELETE.into(), description: "Delete patient".into(), service: service.into() },
        // encounter
        PermissionDef { name: ENCOUNTER_LIST.into(),   description: "List encounters".into(), service: service.into() },
        PermissionDef { name: ENCOUNTER_READ.into(),   description: "Read encounter".into(), service: service.into() },
        PermissionDef { name: ENCOUNTER_CREATE.into(), description: "Create encounter".into(), service: service.into() },
        PermissionDef { name: ENCOUNTER_UPDATE.into(), description: "Update encounter".into(), service: service.into() },
        PermissionDef { name: ENCOUNTER_CLOSE.into(),  description: "Close encounter".into(),  service: service.into() },
    ]
}
