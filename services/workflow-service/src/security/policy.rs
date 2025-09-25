pub mod perm {
    pub const TEMPLATE_UPSERT: &str = "his.workflow.template.upsert";
    pub const TEMPLATE_GET: &str = "his.workflow.template.get";
    pub const INSTANCE_START: &str = "his.workflow.instance.start";
    pub const INSTANCE_GET: &str = "his.workflow.instance.get";
    pub const TASK_CLAIM: &str = "his.workflow.task.claim";
    pub const TASK_COMPLETE: &str = "his.workflow.task.complete";
    pub const OBSERVABILITY_HEALTH: &str = "his.workflow.observability.health";
    pub const EVENT_HANDLE: &str = "his.workflow.event.handle";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(TEMPLATE_UPSERT, "Upsert workflow template", "templates", "upsert"),
        PermissionDef::new(TEMPLATE_GET, "Get workflow template", "templates", "get"),
        PermissionDef::new(INSTANCE_START, "Start workflow instance", "instances", "start"),
        PermissionDef::new(INSTANCE_GET, "Get workflow instance", "instances", "get"),
        PermissionDef::new(TASK_CLAIM, "Claim workflow task", "tasks", "claim"),
        PermissionDef::new(TASK_COMPLETE, "Complete workflow task", "tasks", "complete"),
        PermissionDef::new(OBSERVABILITY_HEALTH, "Check workflow health", "observability", "health"),
        PermissionDef::new(EVENT_HANDLE, "Handle workflow events", "events", "handle"),
    ]
}
