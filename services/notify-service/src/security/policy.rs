pub mod perm {
    // Templates
    pub const TEMPLATE_LIST: &str = "his.notify.template.list";
    pub const TEMPLATE_CREATE: &str = "his.notify.template.create";
    pub const TEMPLATE_UPDATE: &str = "his.notify.template.update";
    pub const TEMPLATE_RENDER: &str = "his.notify.template.render";

    // Messages
    pub const MESSAGE_SEND: &str = "his.notify.message.send";
    pub const MESSAGE_GET: &str = "his.notify.message.get";
    pub const MESSAGE_LIST: &str = "his.notify.message.list";

    // Webhooks
    pub const WEBHOOK_REGISTER: &str = "his.notify.webhook.register";
    pub const WEBHOOK_TRIGGER: &str = "his.notify.webhook.trigger";

    // Notifications (aligned with root.sql)
    pub const NOTIFICATION_LIST: &str = "his.notify.notification.list";
    pub const NOTIFICATION_CREATE: &str = "his.notify.notification.create";
    pub const NOTIFICATION_GET: &str = "his.notify.notification.get";
    pub const NOTIFICATION_UPDATE: &str = "his.notify.notification.update";
    pub const NOTIFICATION_DELETE: &str = "his.notify.notification.delete";
    pub const NOTIFICATION_ASSIGN: &str = "his.notify.notification.assign";
    pub const NOTIFICATION_READ: &str = "his.notify.notification.read";
    pub const WEBHOOK_LIST: &str = "his.notify.webhook.list";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(TEMPLATE_LIST, "List notification templates", "template", "list"),
        PermissionDef::new(TEMPLATE_CREATE, "Create notification template", "template", "create"),
        PermissionDef::new(TEMPLATE_UPDATE, "Update notification template", "template", "update"),
        PermissionDef::new(TEMPLATE_RENDER, "Render notification template", "template", "render"),
        PermissionDef::new(MESSAGE_SEND, "Send notification message", "message", "send"),
        PermissionDef::new(MESSAGE_GET, "Get notification message", "message", "get"),
        PermissionDef::new(MESSAGE_LIST, "List notification messages", "message", "list"),
        PermissionDef::new(WEBHOOK_REGISTER, "Register webhook", "webhook", "register"),
        PermissionDef::new(WEBHOOK_TRIGGER, "Trigger webhook", "webhook", "trigger"),
        PermissionDef::new(WEBHOOK_LIST, "List webhooks", "webhook", "list"),
    ]
}
