CREATE TABLE IF NOT EXISTS outbox_events (
  id            UUID PRIMARY KEY,
  topic         VARCHAR(128) NOT NULL,
  key           VARCHAR(128) NOT NULL,
  payload       JSONB NOT NULL,
  status        VARCHAR(16) NOT NULL DEFAULT 'PENDING',
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  published_at  TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_outbox_status_created ON outbox_events(status, created_at);
