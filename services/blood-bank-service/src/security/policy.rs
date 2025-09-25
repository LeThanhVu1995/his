pub mod perm {
    pub const BLOOD_REQUEST_CREATE: &str = "his.blood.request.create";
    pub const BLOOD_CROSSMATCH_PERFORM: &str = "his.blood.crossmatch.perform";
    pub const BLOOD_ISSUE_RELEASE: &str = "his.blood.issue.release";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(BLOOD_REQUEST_CREATE, "Create blood request", "blood", "request.create"),
        PermissionDef::new(BLOOD_CROSSMATCH_PERFORM, "Perform crossmatch", "blood", "crossmatch.perform"),
        PermissionDef::new(BLOOD_ISSUE_RELEASE, "Release blood unit", "blood", "issue.release"),
    ]
}


