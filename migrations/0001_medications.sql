CREATE TABLE IF NOT EXISTS medications (
  id UUID PRIMARY KEY,
  code VARCHAR(64) UNIQUE NOT NULL,      -- map master-data code nếu có
  name VARCHAR(255) NOT NULL,
  strength VARCHAR(64),                  -- 500mg, 1g/5ml, ...
  form VARCHAR(64),                      -- tablet, syrup, inj
  route VARCHAR(64),                     -- PO, IV, IM, ...
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
