CREATE TABLE IF NOT EXISTS patients (
  id UUID PRIMARY KEY,
  mrn VARCHAR(64) UNIQUE,                 -- Medical Record Number
  national_id VARCHAR(64),                -- CCCD/CMND/SSN
  passport_no VARCHAR(64),
  full_name VARCHAR(255) NOT NULL,
  first_name VARCHAR(100),
  last_name VARCHAR(100),
  gender VARCHAR(16) NOT NULL,            -- male/female/other
  birth_date DATE,
  phone VARCHAR(32),
  email VARCHAR(120),
  address TEXT,
  blood_type VARCHAR(8),
  marital_status VARCHAR(16),
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_patients_full_name ON patients(full_name);
CREATE INDEX IF NOT EXISTS idx_patients_mrn ON patients(mrn);
