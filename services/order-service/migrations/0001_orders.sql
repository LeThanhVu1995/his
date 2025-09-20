CREATE TABLE IF NOT EXISTS orders (
  id UUID PRIMARY KEY,
  patient_id UUID NOT NULL,
  encounter_id UUID,
  order_no VARCHAR(64) UNIQUE NOT NULL,
  order_type VARCHAR(16) NOT NULL,      -- LAB/IMG/PROC
  status VARCHAR(16) NOT NULL,           -- NEW/APPROVED/INPROGRESS/COMPLETED/CANCELLED
  priority VARCHAR(16),                  -- ROUTINE/URGENT/STAT
  ordered_by VARCHAR(64),                -- user id
  note TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_orders_patient ON orders(patient_id);
CREATE INDEX IF NOT EXISTS idx_orders_encounter ON orders(encounter_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
