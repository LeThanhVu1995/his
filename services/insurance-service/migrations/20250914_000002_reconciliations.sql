CREATE TABLE IF NOT EXISTS reconciliations (
  id UUID PRIMARY KEY,
  batch_no VARCHAR(64) UNIQUE NOT NULL,
  payer VARCHAR(64) NOT NULL,
  period_start DATE NOT NULL,
  period_end DATE NOT NULL,
  total_claims INT NOT NULL DEFAULT 0,
  total_amount NUMERIC(14,2) NOT NULL DEFAULT 0,
  approved_amount NUMERIC(14,2) NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
