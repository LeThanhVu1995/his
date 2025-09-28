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

    // Insurance Payers
    pub const PAYER_CREATE: &str = "his.ins.payer.create";
    pub const PAYER_GET: &str = "his.ins.payer.get";
    pub const PAYER_LIST: &str = "his.ins.payer.list";
    pub const PAYER_UPDATE: &str = "his.ins.payer.update";
    pub const PAYER_DELETE: &str = "his.ins.payer.delete";

    // Insurance Policies
    pub const POLICY_CREATE: &str = "his.ins.policy.create";
    pub const POLICY_GET: &str = "his.ins.policy.get";
    pub const POLICY_LIST: &str = "his.ins.policy.list";
    pub const POLICY_UPDATE: &str = "his.ins.policy.update";
    pub const POLICY_DELETE: &str = "his.ins.policy.delete";
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
        PermissionDef::new(PAYER_CREATE, "Create insurance payer", "payer", "create"),
        PermissionDef::new(PAYER_GET, "Get insurance payer", "payer", "get"),
        PermissionDef::new(PAYER_LIST, "List insurance payers", "payer", "list"),
        PermissionDef::new(PAYER_UPDATE, "Update insurance payer", "payer", "update"),
        PermissionDef::new(PAYER_DELETE, "Delete insurance payer", "payer", "delete"),
        PermissionDef::new(POLICY_CREATE, "Create insurance policy", "policy", "create"),
        PermissionDef::new(POLICY_GET, "Get insurance policy", "policy", "get"),
        PermissionDef::new(POLICY_LIST, "List insurance policies", "policy", "list"),
        PermissionDef::new(POLICY_UPDATE, "Update insurance policy", "policy", "update"),
        PermissionDef::new(POLICY_DELETE, "Delete insurance policy", "policy", "delete"),
    ]
}
