CREATE TABLE IF NOT EXISTS wf_tasks (
  id              UUID PRIMARY KEY,
  instance_id     UUID NOT NULL,
  step_id         VARCHAR(64) NOT NULL,
  name            VARCHAR(255) NOT NULL,
  assignee        UUID,
  candidate_roles TEXT[],
  payload         JSONB NOT NULL,
  status          VARCHAR(32) NOT NULL DEFAULT 'READY',
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  completed_at    TIMESTAMPTZ
);
