CREATE TABLE IF NOT EXISTS lab_specimens (
  id UUID PRIMARY KEY,
  specimen_no VARCHAR(64) UNIQUE NOT NULL,
  order_id UUID,                        -- liên kết từ order-service nếu có
  patient_id UUID NOT NULL,
  encounter_id UUID,
  specimen_type VARCHAR(64) NOT NULL,
  collected_at TIMESTAMPTZ,
  collected_by VARCHAR(64),
  status VARCHAR(16) NOT NULL DEFAULT 'CREATED', -- CREATED/COLLECTED/RECEIVED/REJECTED
  note TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_lab_specimens_patient ON lab_specimens(patient_id);
CREATE INDEX IF NOT EXISTS idx_lab_specimens_status ON lab_specimens(status);
