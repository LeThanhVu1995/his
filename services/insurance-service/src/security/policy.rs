pub mod perm {
    // Eligibility
    pub const ELIGIBILITY_CHECK: &str = "his.ins.eligibility.check";

    // Members
    pub const MEMBER_UPSERT: &str = "his.ins.member.upsert";

    // Claims
    pub const CLAIM_CREATE: &str = "his.ins.claim.create";
    pub const CLAIM_GET: &str = "his.ins.claim.get";
    pub const CLAIM_LIST: &str = "his.ins.claim.list";
    pub const CLAIM_SUBMIT: &str = "his.ins.claim.submit";
    pub const CLAIM_SIGN: &str = "his.ins.claim.sign";
    pub const CLAIM_STATUS: &str = "his.ins.claim.status";

    // Reconciliation
    pub const RECON_CREATE: &str = "his.ins.recon.create";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(ELIGIBILITY_CHECK, "Check insurance eligibility", "eligibility", "check"),
        PermissionDef::new(MEMBER_UPSERT, "Upsert insurance member", "member", "upsert"),
        PermissionDef::new(CLAIM_CREATE, "Create insurance claim", "claim", "create"),
        PermissionDef::new(CLAIM_GET, "Get insurance claim", "claim", "get"),
        PermissionDef::new(CLAIM_LIST, "List insurance claims", "claim", "list"),
        PermissionDef::new(CLAIM_SUBMIT, "Submit insurance claim", "claim", "submit"),
        PermissionDef::new(CLAIM_SIGN, "Sign insurance claim", "claim", "sign"),
        PermissionDef::new(CLAIM_STATUS, "Update claim status", "claim", "status"),
        PermissionDef::new(RECON_CREATE, "Create reconciliation", "reconciliation", "create"),
    ]
}
