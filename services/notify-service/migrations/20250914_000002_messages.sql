CREATE TYPE msg_status AS ENUM ('QUEUED','SENT','FAILED');
CREATE TABLE IF NOT EXISTS notify_messages (
  id UUID PRIMARY KEY,
  template_code VARCHAR(64),
  channel VARCHAR(16) NOT NULL,
  to_addr VARCHAR(255) NOT NULL,
  subject TEXT,
  body TEXT NOT NULL,
  status msg_status NOT NULL DEFAULT 'QUEUED',
  err TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  sent_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS notify_webhooks (
  id UUID PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  url TEXT NOT NULL,
  secret TEXT,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
