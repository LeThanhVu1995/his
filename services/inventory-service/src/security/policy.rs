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

    // UOMs
    pub const UOM_LIST: &str = "his.inv.uom.list";
    pub const UOM_CREATE: &str = "his.inv.uom.create";
    pub const UOM_UPDATE: &str = "his.inv.uom.update";

    // Suppliers
    pub const SUPPLIER_LIST: &str = "his.inv.supplier.list";
    pub const SUPPLIER_CREATE: &str = "his.inv.supplier.create";
    pub const SUPPLIER_UPDATE: &str = "his.inv.supplier.update";

    // Purchase Orders
    pub const PO_LIST: &str = "his.inv.po.list";
    pub const PO_CREATE: &str = "his.inv.po.create";
    pub const PO_UPDATE: &str = "his.inv.po.update";

    // Goods Receipts
    pub const GR_LIST: &str = "his.inv.gr.list";
    pub const GR_CREATE: &str = "his.inv.gr.create";
    pub const GR_UPDATE: &str = "his.inv.gr.update";

    // Stock Transactions
    pub const STOCK_TXN_LIST: &str = "his.inv.stock_txn.list";
    pub const STOCK_TXN_VIEW: &str = "his.inv.stock_txn.view";
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
        PermissionDef::new(UOM_LIST, "List UOMs", "uoms", "list"),
        PermissionDef::new(UOM_CREATE, "Create UOM", "uoms", "create"),
        PermissionDef::new(UOM_UPDATE, "Update UOM", "uoms", "update"),
        PermissionDef::new(SUPPLIER_LIST, "List suppliers", "suppliers", "list"),
        PermissionDef::new(SUPPLIER_CREATE, "Create supplier", "suppliers", "create"),
        PermissionDef::new(SUPPLIER_UPDATE, "Update supplier", "suppliers", "update"),
        PermissionDef::new(PO_LIST, "List purchase orders", "purchase_orders", "list"),
        PermissionDef::new(PO_CREATE, "Create purchase order", "purchase_orders", "create"),
        PermissionDef::new(PO_UPDATE, "Update purchase order", "purchase_orders", "update"),
        PermissionDef::new(GR_LIST, "List goods receipts", "goods_receipts", "list"),
        PermissionDef::new(GR_CREATE, "Create goods receipt", "goods_receipts", "create"),
        PermissionDef::new(GR_UPDATE, "Update goods receipt", "goods_receipts", "update"),
        PermissionDef::new(STOCK_TXN_LIST, "List stock transactions", "stock_transactions", "list"),
        PermissionDef::new(STOCK_TXN_VIEW, "View stock transactions", "stock_transactions", "view"),
    ]
}
