CREATE TABLE IF NOT EXISTS lab_tests (
  id UUID PRIMARY KEY,
  code VARCHAR(64) UNIQUE NOT NULL,     -- ví dụ: CBC, GLU, AST
  name VARCHAR(255) NOT NULL,
  specimen_type VARCHAR(64) NOT NULL,   -- BLOOD, URINE, SERUM...
  unit VARCHAR(32),                     -- g/L, mmol/L...
  ref_low NUMERIC(12,4),
  ref_high NUMERIC(12,4),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
