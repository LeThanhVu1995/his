-- Purchase Order Management
CREATE TABLE IF NOT EXISTS inv_purchase_orders (
    id UUID PRIMARY KEY,
    supplier_id UUID NOT NULL REFERENCES inv_suppliers(id),
    facility_id UUID NOT NULL, -- References org_facility from master data
    po_no VARCHAR(64) NOT NULL UNIQUE,
    status VARCHAR(32) NOT NULL DEFAULT 'DRAFT', -- DRAFT, SUBMITTED, APPROVED, RECEIVED, CANCELLED
    ordered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expected_delivery_date DATE,
    total_amount DECIMAL(18,2) DEFAULT 0,
    currency VARCHAR(16) NOT NULL DEFAULT 'VND',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(36),
    updated_by VARCHAR(36)
);

-- Purchase Order Items
CREATE TABLE IF NOT EXISTS inv_po_items (
    id UUID PRIMARY KEY,
    po_id UUID NOT NULL REFERENCES inv_purchase_orders(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES inv_items(id),
    quantity DECIMAL(18,6) NOT NULL,
    uom_id UUID NOT NULL REFERENCES inv_uom(id),
    unit_price DECIMAL(18,6) NOT NULL,
    total_price DECIMAL(18,6) NOT NULL,
    received_quantity DECIMAL(18,6) DEFAULT 0,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_po_supplier ON inv_purchase_orders(supplier_id);
CREATE INDEX IF NOT EXISTS idx_po_facility ON inv_purchase_orders(facility_id);
CREATE INDEX IF NOT EXISTS idx_po_status ON inv_purchase_orders(status);
CREATE INDEX IF NOT EXISTS idx_po_no ON inv_purchase_orders(po_no);
CREATE INDEX IF NOT EXISTS idx_po_items_po ON inv_po_items(po_id);
CREATE INDEX IF NOT EXISTS idx_po_items_item ON inv_po_items(item_id);
