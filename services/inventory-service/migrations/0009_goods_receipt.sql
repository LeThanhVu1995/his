-- Goods Receipt Note (GRN) Management
CREATE TABLE IF NOT EXISTS inv_goods_receipts (
    id UUID PRIMARY KEY,
    po_id UUID REFERENCES inv_purchase_orders(id),
    warehouse_id UUID NOT NULL REFERENCES warehouses(id),
    grn_no VARCHAR(64) NOT NULL UNIQUE,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    received_by VARCHAR(36),
    status VARCHAR(32) NOT NULL DEFAULT 'DRAFT', -- DRAFT, CONFIRMED, CANCELLED
    total_amount DECIMAL(18,2) DEFAULT 0,
    currency VARCHAR(16) NOT NULL DEFAULT 'VND',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(36),
    updated_by VARCHAR(36)
);

-- Goods Receipt Items
CREATE TABLE IF NOT EXISTS inv_grn_items (
    id UUID PRIMARY KEY,
    grn_id UUID NOT NULL REFERENCES inv_goods_receipts(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES inv_items(id),
    batch_id UUID REFERENCES inv_lots(id),
    quantity DECIMAL(18,6) NOT NULL,
    uom_id UUID NOT NULL REFERENCES inv_uom(id),
    unit_price DECIMAL(18,6),
    total_price DECIMAL(18,6),
    expiry_date DATE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_grn_po ON inv_goods_receipts(po_id);
CREATE INDEX IF NOT EXISTS idx_grn_warehouse ON inv_goods_receipts(warehouse_id);
CREATE INDEX IF NOT EXISTS idx_grn_status ON inv_goods_receipts(status);
CREATE INDEX IF NOT EXISTS idx_grn_no ON inv_goods_receipts(grn_no);
CREATE INDEX IF NOT EXISTS idx_grn_items_grn ON inv_grn_items(grn_id);
CREATE INDEX IF NOT EXISTS idx_grn_items_item ON inv_grn_items(item_id);
CREATE INDEX IF NOT EXISTS idx_grn_items_batch ON inv_grn_items(batch_id);
