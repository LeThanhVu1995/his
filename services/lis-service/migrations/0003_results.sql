CREATE TABLE IF NOT EXISTS lab_results (
  id UUID PRIMARY KEY,
  result_no VARCHAR(64) UNIQUE NOT NULL,
  specimen_id UUID NOT NULL REFERENCES lab_specimens(id) ON DELETE CASCADE,
  test_id UUID NOT NULL REFERENCES lab_tests(id),
  status VARCHAR(16) NOT NULL DEFAULT 'NEW',        -- NEW/INPROGRESS/VERIFIED/RELEASED
  verified_by VARCHAR(64),
  verified_at TIMESTAMPTZ,
  released_by VARCHAR(64),
  released_at TIMESTAMPTZ,
  note TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_lab_results_specimen ON lab_results(specimen_id);
CREATE INDEX IF NOT EXISTS idx_lab_results_status ON lab_results(status);
