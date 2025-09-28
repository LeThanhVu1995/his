-- Radiology Results
-- Based on root.sql schema

CREATE TABLE IF NOT EXISTS rad_result (
    rad_result_id   VARCHAR(36) PRIMARY KEY,
    rad_order_item_id VARCHAR(36) NOT NULL,
    report_text     TEXT,
    result_status   VARCHAR(32) NOT NULL DEFAULT 'FINAL',
    reported_at     TIMESTAMP,
    reported_by     VARCHAR(36),
    pacs_study_uid  VARCHAR(128),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_rr_item FOREIGN KEY (rad_order_item_id) REFERENCES rad_order_item(rad_order_item_id),
    CONSTRAINT fk_rr_reporter FOREIGN KEY (reported_by) REFERENCES staff(staff_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_rad_result_item ON rad_result(rad_order_item_id);
CREATE INDEX IF NOT EXISTS idx_rad_result_status ON rad_result(result_status);
CREATE INDEX IF NOT EXISTS idx_rad_result_reported ON rad_result(reported_at);
