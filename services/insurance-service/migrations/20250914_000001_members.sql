CREATE TABLE IF NOT EXISTS ins_members (
  id UUID PRIMARY KEY,
  patient_id UUID NOT NULL,
  payer VARCHAR(64) NOT NULL,     -- BHYT / <private name>
  policy_no VARCHAR(64) NOT NULL,
  plan_code VARCHAR(64),
  start_date DATE,
  end_date DATE,
  status VARCHAR(16) NOT NULL DEFAULT 'ACTIVE',
  holder_name VARCHAR(128),
  note TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(payer, policy_no)
);
CREATE INDEX IF NOT EXISTS idx_ins_members_patient ON ins_members(patient_id);
