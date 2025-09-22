pub mod perm {
    // Catalog
    pub const PROC_LIST:   &str = "his.ris.proc.list";
    pub const PROC_CREATE: &str = "his.ris.proc.create";
    pub const PROC_UPDATE: &str = "his.ris.proc.update";

    // Orders
    pub const ORDER_LIST:   &str = "his.ris.order.list";
    pub const ORDER_CREATE: &str = "his.ris.order.create";
    pub const ORDER_UPDATE: &str = "his.ris.order.update"; // schedule/cancel

    // Studies
    pub const STUDY_LIST:   &str = "his.ris.study.list";
    pub const STUDY_CREATE: &str = "his.ris.study.create";
    pub const STUDY_PROGRESS:&str= "his.ris.study.progress"; // start/end

    // Reports
    pub const REPORT_LIST:   &str = "his.ris.report.list";
    pub const REPORT_CREATE: &str = "his.ris.report.create";
    pub const REPORT_EDIT:   &str = "his.ris.report.edit";    // viết nội dung
    pub const REPORT_VERIFY: &str = "his.ris.report.verify";  // prelim
    pub const REPORT_FINAL:  &str = "his.ris.report.final";   // final
}

use app_web::security::PermissionDef;

pub fn permission_catalog(svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(PROC_LIST, "List procedures", "procedures", "list"),
        PermissionDef::new(PROC_CREATE, "Create procedure", "procedures", "create"),
        PermissionDef::new(PROC_UPDATE, "Update procedure", "procedures", "update"),
        PermissionDef::new(ORDER_LIST, "List orders", "orders", "list"),
        PermissionDef::new(ORDER_CREATE, "Create order", "orders", "create"),
        PermissionDef::new(ORDER_UPDATE, "Update order", "orders", "update"),
        PermissionDef::new(STUDY_LIST, "List studies", "studies", "list"),
        PermissionDef::new(STUDY_CREATE, "Create study", "studies", "create"),
        PermissionDef::new(STUDY_PROGRESS, "Progress study", "studies", "progress"),
        PermissionDef::new(REPORT_LIST, "List reports", "reports", "list"),
        PermissionDef::new(REPORT_CREATE, "Create report", "reports", "create"),
        PermissionDef::new(REPORT_EDIT, "Edit report", "reports", "edit"),
        PermissionDef::new(REPORT_VERIFY, "Verify report", "reports", "verify"),
        PermissionDef::new(REPORT_FINAL, "Finalize report", "reports", "final"),
    ]
}
