CREATE TABLE IF NOT EXISTS wf_templates (
  id              UUID PRIMARY KEY,
  code            VARCHAR(64) NOT NULL UNIQUE,
  name            VARCHAR(255) NOT NULL,
  version         INTEGER NOT NULL DEFAULT 1,
  spec            JSONB NOT NULL,
  is_active       BOOLEAN DEFAULT TRUE,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Note: wf_step and wf_transition are now embedded in wf_templates.spec JSONB
-- This simplifies the schema and makes it more flexible for dynamic workflows
