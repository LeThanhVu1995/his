-- Core notification tables aligned with root.sql

CREATE TABLE IF NOT EXISTS notification (
    notification_id VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64),
    title           VARCHAR(255) NOT NULL,
    body            VARCHAR(1000),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS notification_target (
    notification_id VARCHAR(36) NOT NULL,
    user_id         VARCHAR(36) NOT NULL,
    read_at         TIMESTAMP,
    PRIMARY KEY (notification_id, user_id),
    CONSTRAINT fk_nt_notif FOREIGN KEY (notification_id) REFERENCES notification(notification_id)
    -- Note: user_id references users(user_id) but users table is in IAM service
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_notification_code ON notification(code);
CREATE INDEX IF NOT EXISTS idx_notification_created_at ON notification(created_at);
CREATE INDEX IF NOT EXISTS idx_notification_target_user_id ON notification_target(user_id);
CREATE INDEX IF NOT EXISTS idx_notification_target_read_at ON notification_target(read_at);
