use crate::domain::entities::audit_event::AuditEvent;
use chrono::Utc;
use uuid::Uuid;

pub struct AuditHelper;

impl AuditHelper {
    pub fn create_audit_event(
        user_id: Option<String>,
        entity_name: String,
        entity_id: String,
        action: String,
        before_json: Option<String>,
        after_json: Option<String>,
        ip_address: Option<String>,
    ) -> AuditEvent {
        AuditEvent {
            audit_id: Uuid::new_v4().to_string(),
            event_time: Utc::now(),
            user_id,
            entity_name,
            entity_id,
            action,
            before_json,
            after_json,
            ip_address,
        }
    }

    pub fn create_create_event(
        user_id: Option<String>,
        entity_name: String,
        entity_id: String,
        after_json: Option<String>,
        ip_address: Option<String>,
    ) -> AuditEvent {
        Self::create_audit_event(
            user_id,
            entity_name,
            entity_id,
            "CREATE".to_string(),
            None,
            after_json,
            ip_address,
        )
    }

    pub fn create_update_event(
        user_id: Option<String>,
        entity_name: String,
        entity_id: String,
        before_json: Option<String>,
        after_json: Option<String>,
        ip_address: Option<String>,
    ) -> AuditEvent {
        Self::create_audit_event(
            user_id,
            entity_name,
            entity_id,
            "UPDATE".to_string(),
            before_json,
            after_json,
            ip_address,
        )
    }

    pub fn create_delete_event(
        user_id: Option<String>,
        entity_name: String,
        entity_id: String,
        before_json: Option<String>,
        ip_address: Option<String>,
    ) -> AuditEvent {
        Self::create_audit_event(
            user_id,
            entity_name,
            entity_id,
            "DELETE".to_string(),
            before_json,
            None,
            ip_address,
        )
    }
}
