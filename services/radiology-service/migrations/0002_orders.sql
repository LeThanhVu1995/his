CREATE TABLE IF NOT EXISTS rad_orders (
  id UUID PRIMARY KEY,
  order_no VARCHAR(64) UNIQUE NOT NULL,
  patient_id UUID NOT NULL,
  encounter_id UUID,
  procedure_id UUID NOT NULL REFERENCES rad_procedures(id),
  reason TEXT,
  priority VARCHAR(16) NOT NULL DEFAULT 'ROUTINE',
  status VARCHAR(16) NOT NULL DEFAULT 'NEW', -- NEW/SCHEDULED/CANCELLED/COMPLETED
  requested_by VARCHAR(64),
  scheduled_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_rad_orders_patient ON rad_orders(patient_id);
CREATE INDEX IF NOT EXISTS idx_rad_orders_status ON rad_orders(status);
