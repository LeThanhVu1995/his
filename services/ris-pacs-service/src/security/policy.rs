use app_web::security::PermissionDef;

pub mod perm {
    // Imaging Orders
    pub const ORDER_LIST:    &str = "his.ris_pacs.order.list";
    pub const ORDER_CREATE:  &str = "his.ris_pacs.order.create";
    pub const ORDER_UPDATE:  &str = "his.ris_pacs.order.update";
    pub const ORDER_DELETE:  &str = "his.ris_pacs.order.delete";
    
    // Studies
    pub const STUDY_LIST:    &str = "his.ris_pacs.study.list";
    pub const STUDY_CREATE:  &str = "his.ris_pacs.study.create";
    pub const STUDY_UPDATE:  &str = "his.ris_pacs.study.update";
    pub const STUDY_START:   &str = "his.ris_pacs.study.start";
    pub const STUDY_COMPLETE: &str = "his.ris_pacs.study.complete";
    
    // Reports
    pub const REPORT_LIST:   &str = "his.ris_pacs.report.list";
    pub const REPORT_CREATE: &str = "his.ris_pacs.report.create";
    pub const REPORT_UPDATE: &str = "his.ris_pacs.report.update";
    pub const REPORT_FINAL:  &str = "his.ris_pacs.report.final";
}

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(ORDER_LIST, "List imaging orders", "orders", "list"),
        PermissionDef::new(ORDER_CREATE, "Create imaging order", "orders", "create"),
        PermissionDef::new(ORDER_UPDATE, "Update imaging order", "orders", "update"),
        PermissionDef::new(ORDER_DELETE, "Delete imaging order", "orders", "delete"),
        
        PermissionDef::new(STUDY_LIST, "List studies", "studies", "list"),
        PermissionDef::new(STUDY_CREATE, "Create study", "studies", "create"),
        PermissionDef::new(STUDY_UPDATE, "Update study", "studies", "update"),
        PermissionDef::new(STUDY_START, "Start study", "studies", "start"),
        PermissionDef::new(STUDY_COMPLETE, "Complete study", "studies", "complete"),
        
        PermissionDef::new(REPORT_LIST, "List reports", "reports", "list"),
        PermissionDef::new(REPORT_CREATE, "Create report", "reports", "create"),
        PermissionDef::new(REPORT_UPDATE, "Update report", "reports", "update"),
        PermissionDef::new(REPORT_FINAL, "Finalize report", "reports", "final"),
    ]
}
