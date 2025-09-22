CREATE TABLE IF NOT EXISTS rad_reports (
  id UUID PRIMARY KEY,
  report_no VARCHAR(64) UNIQUE NOT NULL,
  study_id UUID NOT NULL REFERENCES rad_studies(id) ON DELETE CASCADE,
  status VARCHAR(16) NOT NULL DEFAULT 'DRAFT', -- DRAFT/PRELIM/FINAL
  content TEXT,
  author VARCHAR(64),
  verified_by VARCHAR(64),
  verified_at TIMESTAMPTZ,
  finalized_by VARCHAR(64),
  finalized_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_rad_reports_study ON rad_reports(study_id);
