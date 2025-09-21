CREATE TABLE IF NOT EXISTS prescriptions (
  id UUID PRIMARY KEY,
  patient_id UUID NOT NULL,
  encounter_id UUID,
  presc_no VARCHAR(64) UNIQUE NOT NULL,
  status VARCHAR(16) NOT NULL DEFAULT 'NEW', -- NEW/APPROVED/DISPENSING/COMPLETED/CANCELLED
  ordered_by VARCHAR(64),
  note TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_prescriptions_patient ON prescriptions(patient_id);
CREATE INDEX IF NOT EXISTS idx_prescriptions_status ON prescriptions(status);
