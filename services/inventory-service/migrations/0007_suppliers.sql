-- Supplier Management
CREATE TABLE IF NOT EXISTS inv_suppliers (
    id UUID PRIMARY KEY,
    code VARCHAR(64) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(32),
    email VARCHAR(128),
    address_line1 VARCHAR(255),
    address_line2 VARCHAR(255),
    city VARCHAR(255),
    province VARCHAR(255),
    country VARCHAR(64),
    postal_code VARCHAR(32),
    tax_id VARCHAR(64),
    status VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(36),
    updated_by VARCHAR(36),
    deleted_at TIMESTAMPTZ,
    deleted_by VARCHAR(36)
);

-- Add supplier_id to lots table
ALTER TABLE inv_lots ADD COLUMN IF NOT EXISTS supplier_id UUID REFERENCES inv_suppliers(id);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_suppliers_code ON inv_suppliers(code);
CREATE INDEX IF NOT EXISTS idx_suppliers_status ON inv_suppliers(status);
CREATE INDEX IF NOT EXISTS idx_lots_supplier ON inv_lots(supplier_id);
