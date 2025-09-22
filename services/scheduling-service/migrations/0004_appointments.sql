CREATE TABLE IF NOT EXISTS appointments (
  id UUID PRIMARY KEY,
  appt_no VARCHAR(64) UNIQUE NOT NULL,
  patient_id UUID NOT NULL,
  provider_id UUID NOT NULL REFERENCES providers(id),
  room_id UUID REFERENCES rooms(id),
  slot_id UUID NOT NULL REFERENCES time_slots(id) ON DELETE RESTRICT,
  status VARCHAR(16) NOT NULL DEFAULT 'BOOKED', -- BOOKED/CANCELLED/COMPLETED/NO_SHOW
  reason TEXT,
  created_by VARCHAR(64),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_appt_patient ON appointments(patient_id);
