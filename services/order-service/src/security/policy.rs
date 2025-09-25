pub mod perm {
    pub const ORDER_LIST: &str = "his.order.list";
    pub const ORDER_READ: &str = "his.order.read";
    pub const ORDER_CREATE: &str = "his.order.create";
    pub const ORDER_UPDATE: &str = "his.order.update";
    pub const ORDER_CANCEL: &str = "his.order.cancel";
    pub const ORDER_ITEM_ADD: &str = "his.order.item.add";
    pub const ORDER_ITEM_UPDATE: &str = "his.order.item.update";
    pub const ORDER_ITEM_RESULT: &str = "his.order.item.result";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(_service_name: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(ORDER_LIST, "List orders", "orders", "list"),
        PermissionDef::new(ORDER_READ, "Read order", "orders", "read"),
        PermissionDef::new(ORDER_CREATE, "Create order", "orders", "create"),
        PermissionDef::new(ORDER_UPDATE, "Update order", "orders", "update"),
        PermissionDef::new(ORDER_CANCEL, "Cancel order", "orders", "cancel"),
        PermissionDef::new(ORDER_ITEM_ADD, "Add order item", "order_items", "add"),
        PermissionDef::new(ORDER_ITEM_UPDATE, "Update order item", "order_items", "update"),
        PermissionDef::new(ORDER_ITEM_RESULT, "Submit order item result", "order_items", "result"),
    ]
}


