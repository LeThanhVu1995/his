-- Stock Transaction Audit Trail
CREATE TABLE IF NOT EXISTS inv_stock_transactions (
    id UUID PRIMARY KEY,
    warehouse_id UUID NOT NULL REFERENCES warehouses(id),
    item_id UUID NOT NULL REFERENCES inv_items(id),
    batch_id UUID REFERENCES inv_lots(id),
    qty_delta DECIMAL(18,6) NOT NULL, -- positive for increase, negative for decrease
    uom_id UUID NOT NULL REFERENCES inv_uom(id),
    reason_code VARCHAR(64) NOT NULL, -- GRN, ISSUE, ADJUST, RETURN, TRANSFER, DISPENSE
    ref_entity VARCHAR(64), -- e.g., grn_id, movement_id, dispense_id
    ref_id UUID,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    performed_by VARCHAR(36),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enhanced warehouse table with facility reference
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS facility_id UUID; -- References org_facility
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS address_line1 VARCHAR(255);
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS address_line2 VARCHAR(255);
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS city VARCHAR(255);
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS province VARCHAR(255);
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS country VARCHAR(64);
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS postal_code VARCHAR(32);
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS status VARCHAR(32) NOT NULL DEFAULT 'ACTIVE';
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS created_by VARCHAR(36);
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS updated_by VARCHAR(36);
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE warehouses ADD COLUMN IF NOT EXISTS deleted_by VARCHAR(36);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_stock_txn_warehouse ON inv_stock_transactions(warehouse_id);
CREATE INDEX IF NOT EXISTS idx_stock_txn_item ON inv_stock_transactions(item_id);
CREATE INDEX IF NOT EXISTS idx_stock_txn_batch ON inv_stock_transactions(batch_id);
CREATE INDEX IF NOT EXISTS idx_stock_txn_reason ON inv_stock_transactions(reason_code);
CREATE INDEX IF NOT EXISTS idx_stock_txn_ref ON inv_stock_transactions(ref_entity, ref_id);
CREATE INDEX IF NOT EXISTS idx_stock_txn_date ON inv_stock_transactions(occurred_at);
CREATE INDEX IF NOT EXISTS idx_warehouses_facility ON warehouses(facility_id);
CREATE INDEX IF NOT EXISTS idx_warehouses_status ON warehouses(status);
