CREATE TABLE IF NOT EXISTS time_slots (
  id UUID PRIMARY KEY,
  provider_id UUID NOT NULL REFERENCES providers(id) ON DELETE CASCADE,
  room_id UUID REFERENCES rooms(id) ON DELETE SET NULL,
  starts_at TIMESTAMPTZ NOT NULL,
  ends_at   TIMESTAMPTZ NOT NULL,
  reserved  BOOLEAN NOT NULL DEFAULT FALSE,
  locked_by VARCHAR(64),
  UNIQUE(provider_id, starts_at, ends_at)
);
CREATE INDEX IF NOT EXISTS idx_slots_provider_time ON time_slots(provider_id, starts_at);
