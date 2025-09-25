CREATE TABLE IF NOT EXISTS wf_saga_log (
  id            UUID PRIMARY KEY,
  instance_id   UUID NOT NULL,
  step_id       VARCHAR(64) NOT NULL,
  action        VARCHAR(32) NOT NULL,
  request       JSONB,
  response      JSONB,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
