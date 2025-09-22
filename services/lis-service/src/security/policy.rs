pub mod perm {
    // Catalog (lab_tests)
    pub const TEST_LIST: &str = "his.lab.test.list";
    pub const TEST_CREATE: &str = "his.lab.test.create";
    pub const TEST_UPDATE: &str = "his.lab.test.update";

    // Specimens
    pub const SPECIMEN_LIST: &str = "his.lab.specimen.list";
    pub const SPECIMEN_CREATE: &str = "his.lab.specimen.create";
    pub const SPECIMEN_COLLECT: &str = "his.lab.specimen.collect";
    pub const SPECIMEN_RECEIVE: &str = "his.lab.specimen.receive";
    pub const SPECIMEN_REJECT: &str = "his.lab.specimen.reject";

    // Results
    pub const RESULT_LIST: &str = "his.lab.result.list";
    pub const RESULT_CREATE: &str = "his.lab.result.create";
    pub const RESULT_ENTER: &str = "his.lab.result.enter";    // nhập giá trị
    pub const RESULT_VERIFY: &str = "his.lab.result.verify";   // duyệt kết quả
    pub const RESULT_RELEASE: &str = "his.lab.result.release";  // phát hành
}

#[derive(serde::Serialize)]
pub struct PermissionDef {
    pub name: String,
    pub description: String,
    pub service: String,
}

pub fn permission_catalog(svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef { name: TEST_LIST.into(), description: "List lab tests".into(), service: svc.into() },
        PermissionDef { name: TEST_CREATE.into(), description: "Create lab test".into(), service: svc.into() },
        PermissionDef { name: TEST_UPDATE.into(), description: "Update lab test".into(), service: svc.into() },
        PermissionDef { name: SPECIMEN_LIST.into(), description: "List specimens".into(), service: svc.into() },
        PermissionDef { name: SPECIMEN_CREATE.into(), description: "Create specimen".into(), service: svc.into() },
        PermissionDef { name: SPECIMEN_COLLECT.into(), description: "Collect specimen".into(), service: svc.into() },
        PermissionDef { name: SPECIMEN_RECEIVE.into(), description: "Receive specimen".into(), service: svc.into() },
        PermissionDef { name: SPECIMEN_REJECT.into(), description: "Reject specimen".into(), service: svc.into() },
        PermissionDef { name: RESULT_LIST.into(), description: "List results".into(), service: svc.into() },
        PermissionDef { name: RESULT_CREATE.into(), description: "Create result".into(), service: svc.into() },
        PermissionDef { name: RESULT_ENTER.into(), description: "Enter result values".into(), service: svc.into() },
        PermissionDef { name: RESULT_VERIFY.into(), description: "Verify results".into(), service: svc.into() },
        PermissionDef { name: RESULT_RELEASE.into(), description: "Release results".into(), service: svc.into() },
    ]
}
