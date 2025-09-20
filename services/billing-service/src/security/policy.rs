pub mod perm {
    // Charges
    pub const CHARGE_LIST:   &str = "his.billing.charge.list";
    pub const CHARGE_CREATE: &str = "his.billing.charge.create";
    pub const CHARGE_UPDATE: &str = "his.billing.charge.update";
    pub const CHARGE_VOID:   &str = "his.billing.charge.void";
    // Invoices
    pub const INVOICE_LIST:   &str = "his.billing.invoice.list";
    pub const INVOICE_READ:   &str = "his.billing.invoice.read";
    pub const INVOICE_CREATE: &str = "his.billing.invoice.create";  // generate
    pub const INVOICE_ISSUE:  &str = "his.billing.invoice.issue";   // chuyển DRAFT→ISSUED
    pub const INVOICE_VOID:   &str = "his.billing.invoice.void";
    // Payments
    pub const PAYMENT_LIST:   &str = "his.billing.payment.list";
    pub const PAYMENT_CREATE: &str = "his.billing.payment.create";
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
        PermissionDef { name: CHARGE_LIST.into(),   description: "List charges".into(),   service: svc.into() },
        PermissionDef { name: CHARGE_CREATE.into(), description: "Create charge".into(), service: svc.into() },
        PermissionDef { name: CHARGE_UPDATE.into(), description: "Update charge".into(), service: svc.into() },
        PermissionDef { name: CHARGE_VOID.into(),   description: "Void charge".into(),   service: svc.into() },
        PermissionDef { name: INVOICE_LIST.into(),  description: "List invoices".into(), service: svc.into() },
        PermissionDef { name: INVOICE_READ.into(),  description: "Read invoice".into(),  service: svc.into() },
        PermissionDef { name: INVOICE_CREATE.into(),description: "Create invoice".into(),service: svc.into() },
        PermissionDef { name: INVOICE_ISSUE.into(), description: "Issue invoice".into(), service: svc.into() },
        PermissionDef { name: INVOICE_VOID.into(),  description: "Void invoice".into(),  service: svc.into() },
        PermissionDef { name: PAYMENT_LIST.into(),  description: "List payments".into(), service: svc.into() },
        PermissionDef { name: PAYMENT_CREATE.into(),description: "Create payment".into(),service: svc.into() },
    ]
}

