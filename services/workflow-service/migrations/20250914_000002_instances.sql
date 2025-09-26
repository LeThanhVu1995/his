CREATE TABLE IF NOT EXISTS wf_instances (
  id              UUID PRIMARY KEY,
  template_code   VARCHAR(64) NOT NULL,
  template_version INTEGER NOT NULL DEFAULT 1,
  status          VARCHAR(32) NOT NULL DEFAULT 'PENDING',
  input           JSONB NOT NULL,
  context         JSONB NOT NULL,
  cursor          JSONB NOT NULL,
  error           TEXT,
  next_wake_at    TIMESTAMPTZ,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
