pub mod perm {
    pub const OR_SCHEDULE_CREATE: &str = "his.or.schedule.create";
    pub const OR_SCHEDULE_VIEW: &str = "his.or.schedule.view";
    pub const OR_PROCEDURE_START: &str = "his.or.procedure.start";
    pub const OR_PROCEDURE_COMPLETE: &str = "his.or.procedure.complete";
    pub const CSSD_STERILIZE: &str = "his.cssd.sterilize";
    pub const CSSD_ISSUE: &str = "his.cssd.issue";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(OR_SCHEDULE_CREATE, "Create OR schedule", "or", "schedule.create"),
        PermissionDef::new(OR_SCHEDULE_VIEW, "View OR schedules", "or", "schedule.view"),
        PermissionDef::new(OR_PROCEDURE_START, "Start procedure", "or", "procedure.start"),
        PermissionDef::new(OR_PROCEDURE_COMPLETE, "Complete procedure", "or", "procedure.complete"),
        PermissionDef::new(CSSD_STERILIZE, "Sterilize tray", "cssd", "sterilize"),
        PermissionDef::new(CSSD_ISSUE, "Issue tray", "cssd", "issue"),
    ]
}


