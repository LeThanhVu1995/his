pub mod perm {
    // Warehouses
    pub const WH_LIST: &str = "his.inv.wh.list";
    pub const WH_CREATE: &str = "his.inv.wh.create";
    pub const WH_UPDATE: &str = "his.inv.wh.update";

    // Items & Lots
    pub const ITEM_LIST: &str = "his.inv.item.list";
    pub const ITEM_CREATE: &str = "his.inv.item.create";
    pub const ITEM_UPDATE: &str = "his.inv.item.update";
    pub const LOT_LIST: &str = "his.inv.lot.list";
    pub const LOT_CREATE: &str = "his.inv.lot.create";

    // Stocks
    pub const STOCK_VIEW: &str = "his.inv.stock.view";

    // Movements
    pub const MOVE_LIST: &str = "his.inv.move.list";
    pub const MOVE_RECEIVE: &str = "his.inv.move.receive";
    pub const MOVE_ISSUE: &str = "his.inv.move.issue";
    pub const MOVE_ADJUST: &str = "his.inv.move.adjust";
    pub const MOVE_TRANSFER: &str = "his.inv.move.transfer";
}

pub use app_web::security::PermissionDef;

pub fn permission_catalog(svc: &str) -> Vec<PermissionDef> {
    use perm::*;
    vec![
        PermissionDef::new(WH_LIST, "List warehouses", "warehouses", "list"),
        PermissionDef::new(WH_CREATE, "Create warehouse", "warehouses", "create"),
        PermissionDef::new(WH_UPDATE, "Update warehouse", "warehouses", "update"),
        PermissionDef::new(ITEM_LIST, "List items", "items", "list"),
        PermissionDef::new(ITEM_CREATE, "Create item", "items", "create"),
        PermissionDef::new(ITEM_UPDATE, "Update item", "items", "update"),
        PermissionDef::new(LOT_LIST, "List lots", "lots", "list"),
        PermissionDef::new(LOT_CREATE, "Create lot", "lots", "create"),
        PermissionDef::new(STOCK_VIEW, "View stocks", "stocks", "view"),
        PermissionDef::new(MOVE_LIST, "List movements", "movements", "list"),
        PermissionDef::new(MOVE_RECEIVE, "Receive stocks", "movements", "receive"),
        PermissionDef::new(MOVE_ISSUE, "Issue stocks", "movements", "issue"),
        PermissionDef::new(MOVE_ADJUST, "Adjust stocks", "movements", "adjust"),
        PermissionDef::new(MOVE_TRANSFER, "Transfer stocks", "movements", "transfer"),
    ]
}
