pub mod perm {
    // OR Cases (aligned with root.sql)
    pub const OR_CASE_LIST: &str = "his.or.case.list";
    pub const OR_CASE_CREATE: &str = "his.or.case.create";
    pub const OR_CASE_GET: &str = "his.or.case.get";
    pub const OR_CASE_UPDATE: &str = "his.or.case.update";
    pub const OR_CASE_DELETE: &str = "his.or.case.delete";
    pub const OR_CASE_VIEW: &str = "his.or.case.view";
    pub const OR_CHECKLIST_MANAGE: &str = "his.or.checklist.manage";

    // CSSD Trays (aligned with root.sql)
    pub const CSSD_TRAY_LIST: &str = "his.cssd.tray.list";
    pub const CSSD_TRAY_CREATE: &str = "his.cssd.tray.create";
    pub const CSSD_TRAY_GET: &str = "his.cssd.tray.get";
    pub const CSSD_TRAY_UPDATE: &str = "his.cssd.tray.update";
    pub const CSSD_TRAY_DELETE: &str = "his.cssd.tray.delete";
    pub const CSSD_TRAY_ITEM_MANAGE: &str = "his.cssd.tray.item.manage";

    // CSSD Sterilization Lots (aligned with root.sql)
    pub const CSSD_LOT_LIST: &str = "his.cssd.lot.list";
    pub const CSSD_LOT_CREATE: &str = "his.cssd.lot.create";
    pub const CSSD_LOT_GET: &str = "his.cssd.lot.get";
    pub const CSSD_LOT_UPDATE: &str = "his.cssd.lot.update";
    pub const CSSD_LOT_DELETE: &str = "his.cssd.lot.delete";
    pub const CSSD_LOT_ITEM_MANAGE: &str = "his.cssd.lot.item.manage";
    pub const CSSD_VIEW: &str = "his.cssd.view";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        // OR Cases (aligned with root.sql)
        PermissionDef::new(OR_CASE_LIST, "List OR cases", "or", "case.list"),
        PermissionDef::new(OR_CASE_CREATE, "Create OR case", "or", "case.create"),
        PermissionDef::new(OR_CASE_GET, "Get OR case", "or", "case.get"),
        PermissionDef::new(OR_CASE_UPDATE, "Update OR case", "or", "case.update"),
        PermissionDef::new(OR_CASE_DELETE, "Delete OR case", "or", "case.delete"),
        PermissionDef::new(OR_CASE_VIEW, "View OR case", "or", "case.view"),
        PermissionDef::new(OR_CHECKLIST_MANAGE, "Manage OR checklist", "or", "checklist.manage"),

        // CSSD Trays (aligned with root.sql)
        PermissionDef::new(CSSD_TRAY_LIST, "List CSSD trays", "cssd", "tray.list"),
        PermissionDef::new(CSSD_TRAY_CREATE, "Create CSSD tray", "cssd", "tray.create"),
        PermissionDef::new(CSSD_TRAY_GET, "Get CSSD tray", "cssd", "tray.get"),
        PermissionDef::new(CSSD_TRAY_UPDATE, "Update CSSD tray", "cssd", "tray.update"),
        PermissionDef::new(CSSD_TRAY_DELETE, "Delete CSSD tray", "cssd", "tray.delete"),
        PermissionDef::new(CSSD_TRAY_ITEM_MANAGE, "Manage CSSD tray items", "cssd", "tray.item.manage"),

        // CSSD Sterilization Lots (aligned with root.sql)
        PermissionDef::new(CSSD_LOT_LIST, "List CSSD sterilization lots", "cssd", "lot.list"),
        PermissionDef::new(CSSD_LOT_CREATE, "Create CSSD sterilization lot", "cssd", "lot.create"),
        PermissionDef::new(CSSD_LOT_GET, "Get CSSD sterilization lot", "cssd", "lot.get"),
        PermissionDef::new(CSSD_LOT_UPDATE, "Update CSSD sterilization lot", "cssd", "lot.update"),
        PermissionDef::new(CSSD_LOT_DELETE, "Delete CSSD sterilization lot", "cssd", "lot.delete"),
        PermissionDef::new(CSSD_LOT_ITEM_MANAGE, "Manage CSSD lot items", "cssd", "lot.item.manage"),
        PermissionDef::new(CSSD_VIEW, "View CSSD", "cssd", "view"),
    ]
}


