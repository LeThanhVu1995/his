CREATE TYPE claim_status AS ENUM ('DRAFT','CREATED','SUBMITTED','SIGNED','REJECTED','APPROVED','PAID','VOID');
CREATE TABLE IF NOT EXISTS claims (
  id UUID PRIMARY KEY,
  claim_no VARCHAR(64) UNIQUE NOT NULL,
  patient_id UUID NOT NULL,
  encounter_id UUID,
  member_id UUID NOT NULL REFERENCES ins_members(id) ON DELETE RESTRICT,
  payer VARCHAR(64) NOT NULL,
  total_amount NUMERIC(14,2) NOT NULL DEFAULT 0,
  currency VARCHAR(8) NOT NULL DEFAULT 'VND',
  status claim_status NOT NULL DEFAULT 'CREATED',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS claim_items (
  id UUID PRIMARY KEY,
  claim_id UUID NOT NULL REFERENCES claims(id) ON DELETE CASCADE,
  code VARCHAR(32) NOT NULL,   -- service/med code
  description VARCHAR(255),
  qty NUMERIC(12,3) NOT NULL,
  unit_price NUMERIC(12,2) NOT NULL,
  amount NUMERIC(14,2) GENERATED ALWAYS AS (qty * unit_price) STORED,
  coverage_rate NUMERIC(5,2) DEFAULT 100.0,
  patient_pay NUMERIC(14,2) DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_claim_items_claim ON claim_items(claim_id);
