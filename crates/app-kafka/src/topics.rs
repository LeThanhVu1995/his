// src/topics.rs placeholder
//! Định nghĩa tên topic chuẩn cho HIS.
//! Bạn có thể chỉnh tên cho đúng môi trường của mình.
//!
//! Quy ước:
//! - Sử dụng tiền tố `his.`
//! - Dùng số nhiều và hậu tố `.events` cho event bus
//! - Command/request topics dùng `.cmd`/`.req` nếu cần

pub const PATIENT_EVENTS: &str       = "his.patient.events";
pub const ENCOUNTER_EVENTS: &str     = "his.encounter.events";
pub const EMR_EVENTS: &str           = "his.emr.events";
pub const LIS_EVENTS: &str           = "his.lis.events";
pub const RIS_EVENTS: &str           = "his.ris.events";
pub const PHARMACY_EVENTS: &str      = "his.pharmacy.events";
pub const INVENTORY_EVENTS: &str     = "his.inventory.events";
pub const BILLING_EVENTS: &str       = "his.billing.events";
pub const INSURANCE_EVENTS: &str     = "his.insurance.events";
pub const OR_CSSD_EVENTS: &str       = "his.or-cssd.events";
pub const BLOOD_EVENTS: &str         = "his.blood.events";
pub const NOTIFY_EVENTS: &str        = "his.notify.events";
pub const WORKFLOW_EVENTS: &str      = "his.workflow.events";
pub const REPORTING_EVENTS: &str     = "his.reporting.events";
pub const IOT_EVENTS: &str           = "his.iot.events";
pub const DMS_EVENTS: &str           = "his.dms.events";
pub const SEARCH_EVENTS: &str        = "his.search.events";
pub const AUDIT_EVENTS: &str         = "his.audit.events";

/// Ví dụ command topics (tuỳ hệ thống có dùng hay không)
pub const ORDER_CMD: &str            = "his.order.cmd";
pub const CLAIM_CMD: &str            = "his.claim.cmd";

/// Helper nhỏ nếu bạn muốn lấy list nhanh để subscribe toàn bộ.
pub fn all_event_topics() -> &'static [&'static str] {
    &[
        PATIENT_EVENTS,
        ENCOUNTER_EVENTS,
        EMR_EVENTS,
        LIS_EVENTS,
        RIS_EVENTS,
        PHARMACY_EVENTS,
        INVENTORY_EVENTS,
        BILLING_EVENTS,
        INSURANCE_EVENTS,
        OR_CSSD_EVENTS,
        BLOOD_EVENTS,
        NOTIFY_EVENTS,
        WORKFLOW_EVENTS,
        REPORTING_EVENTS,
        IOT_EVENTS,
        DMS_EVENTS,
        SEARCH_EVENTS,
        AUDIT_EVENTS,
    ]
}
