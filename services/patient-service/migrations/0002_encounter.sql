CREATE TABLE IF NOT EXISTS encounters (
  id UUID PRIMARY KEY,
  patient_id UUID NOT NULL REFERENCES patients(id) ON DELETE CASCADE,
  encounter_no VARCHAR(64) UNIQUE NOT NULL,
  encounter_type VARCHAR(16) NOT NULL,      -- OPD/IPD/ER
  status VARCHAR(16) NOT NULL,              -- PLANNED/INPROGRESS/FINISHED/CANCELLED
  department_code VARCHAR(32),              -- liên quan master departments
  attending_doctor_id VARCHAR(64),
  admitted_at TIMESTAMPTZ,
  discharged_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_encounters_patient ON encounters(patient_id);
CREATE INDEX IF NOT EXISTS idx_encounters_status ON encounters(status);
