pub mod perm {
    // Medications catalog (read-only cho dược sĩ)
    pub const MED_LIST: &str = "his.pharmacy.med.list";
    pub const MED_CREATE: &str = "his.pharmacy.med.create"; // optional, cho admin dược
    pub const MED_UPDATE: &str = "his.pharmacy.med.update";

    // Prescriptions
    pub const PRESC_LIST: &str = "his.pharmacy.presc.list";
    pub const PRESC_READ: &str = "his.pharmacy.presc.read";
    pub const PRESC_CREATE: &str = "his.pharmacy.presc.create"; // bác sĩ tạo
    pub const PRESC_UPDATE: &str = "his.pharmacy.presc.update"; // chỉnh sửa khi NEW
    pub const PRESC_APPROVE: &str = "his.pharmacy.presc.approve"; // dược sĩ duyệt
    pub const PRESC_CANCEL: &str = "his.pharmacy.presc.cancel";

    // Dispense
    pub const DISPENSE_LIST: &str = "his.pharmacy.disp.list";
    pub const DISPENSE_CREATE: &str = "his.pharmacy.disp.create"; // lập phiếu cấp phát
    pub const DISPENSE_FINISH: &str = "his.pharmacy.disp.finish"; // hoàn tất
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
        PermissionDef {
            name: MED_LIST.into(),
            description: "List medications".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: MED_CREATE.into(),
            description: "Create medication".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: MED_UPDATE.into(),
            description: "Update medication".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: PRESC_LIST.into(),
            description: "List prescriptions".into(),
            service: svc.into(),

       },
        PermissionDef {
            name: PRESC_READ.into(),
            description: "Read prescription".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: PRESC_CREATE.into(),
            description: "Create prescription".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: PRESC_UPDATE.into(),
            description: "Update prescription".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: PRESC_APPROVE.into(),
            description: "Approve prescription".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: PRESC_CANCEL.into(),
            description: "Cancel prescription".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: DISPENSE_LIST.into(),
            description: "List dispenses".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: DISPENSE_CREATE.into(),
            description: "Create dispense".into(),
            service: svc.into(),
        },
        PermissionDef {
            name: DISPENSE_FINISH.into(),
            description: "Finish dispense".into(),
            service: svc.into(),
        },
    ]
}
