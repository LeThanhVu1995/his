-- Unit of Measure (UOM) Management
CREATE TABLE IF NOT EXISTS inv_uom (
    id UUID PRIMARY KEY,
    code VARCHAR(32) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Item UOM conversion factors
CREATE TABLE IF NOT EXISTS inv_item_uom (
    id UUID PRIMARY KEY,
    item_id UUID NOT NULL REFERENCES inv_items(id) ON DELETE CASCADE,
    uom_id UUID NOT NULL REFERENCES inv_uom(id) ON DELETE CASCADE,
    factor DECIMAL(18,6) NOT NULL, -- conversion factor from base UOM
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(item_id, uom_id)
);

-- Add base_uom_id to items table
ALTER TABLE inv_items ADD COLUMN IF NOT EXISTS base_uom_id UUID REFERENCES inv_uom(id);
ALTER TABLE inv_items ADD COLUMN IF NOT EXISTS category_code VARCHAR(64); -- DRUG, CONSUMABLE, DEVICE, TRAY
ALTER TABLE inv_items ADD COLUMN IF NOT EXISTS is_lot_tracked BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE inv_items ADD COLUMN IF NOT EXISTS is_expirable BOOLEAN NOT NULL DEFAULT TRUE;

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_item_uom_item ON inv_item_uom(item_id);
CREATE INDEX IF NOT EXISTS idx_item_uom_uom ON inv_item_uom(uom_id);
CREATE INDEX IF NOT EXISTS idx_items_base_uom ON inv_items(base_uom_id);
CREATE INDEX IF NOT EXISTS idx_items_category ON inv_items(category_code);
