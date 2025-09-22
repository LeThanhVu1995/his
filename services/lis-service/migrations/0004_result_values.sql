CREATE TABLE IF NOT EXISTS lab_result_values (
  id UUID PRIMARY KEY,
  result_id UUID NOT NULL REFERENCES lab_results(id) ON DELETE CASCADE,
  analyte_code VARCHAR(64) NOT NULL,     -- nếu là panel nhiều chỉ số
  analyte_name VARCHAR(255) NOT NULL,
  value_num NUMERIC(12,4),
  value_text VARCHAR(255),
  unit VARCHAR(32),
  ref_low NUMERIC(12,4),
  ref_high NUMERIC(12,4),
  flag VARCHAR(16),                      -- L/H/CRIT
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_lab_result_values_result ON lab_result_values(result_id);
