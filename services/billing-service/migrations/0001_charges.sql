CREATE TABLE IF NOT EXISTS charges (
  id UUID PRIMARY KEY,
  patient_id UUID NOT NULL,
  encounter_id UUID,
  order_id UUID,
  code VARCHAR(64) NOT NULL,        -- map master code / order item code
  name VARCHAR(255) NOT NULL,
  qty NUMERIC(12,2) NOT NULL DEFAULT 1,
  unit_price NUMERIC(12,2) NOT NULL,
  amount NUMERIC(12,2) NOT NULL,    -- qty * unit_price
  currency VARCHAR(8) NOT NULL DEFAULT 'VND',
  status VARCHAR(16) NOT NULL DEFAULT 'NEW', -- NEW/POSTED/VOID
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_charges_encounter ON charges(encounter_id);
CREATE INDEX IF NOT EXISTS idx_charges_status ON charges(status);

