CREATE TABLE IF NOT EXISTS schedules (
  id UUID PRIMARY KEY,
  provider_id UUID REFERENCES providers(id) ON DELETE CASCADE,
  room_id UUID REFERENCES rooms(id) ON DELETE SET NULL,
  weekday SMALLINT NOT NULL CHECK (weekday BETWEEN 1 AND 7), -- 1=Mon .. 7=Sun
  start_time TIME NOT NULL,
  end_time TIME NOT NULL,
  slot_min SMALLINT NOT NULL DEFAULT 15,
  active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_sched_provider ON schedules(provider_id);
