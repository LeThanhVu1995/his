-- Audit Log Table
CREATE TABLE IF NOT EXISTS audit_log (
    audit_id        VARCHAR(36) PRIMARY KEY,
    event_time      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id         VARCHAR(36),
    entity_name     VARCHAR(64) NOT NULL,
    entity_id       VARCHAR(36) NOT NULL,
    action          VARCHAR(32) NOT NULL,   -- CREATE, UPDATE, DELETE
    before_json     TEXT,
    after_json      TEXT,
    ip_address      VARCHAR(64)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_audit_log_event_time ON audit_log(event_time);
CREATE INDEX IF NOT EXISTS idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_entity ON audit_log(entity_name, entity_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_action ON audit_log(action);
