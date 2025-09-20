-- Master Codes (ví dụ code list dùng chung)
CREATE TABLE IF NOT EXISTS master_codes (
    id            UUID PRIMARY KEY,
    category      VARCHAR(64) NOT NULL,
    code          VARCHAR(64) NOT NULL,
    name          VARCHAR(255) NOT NULL,
    description   TEXT,
    is_active     BOOLEAN NOT NULL DEFAULT TRUE,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (category, code)
);

CREATE INDEX IF NOT EXISTS idx_master_codes_category ON master_codes(category);
CREATE INDEX IF NOT EXISTS idx_master_codes_active ON master_codes(is_active);
