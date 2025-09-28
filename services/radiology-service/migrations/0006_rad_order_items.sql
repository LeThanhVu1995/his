-- Radiology Order Items
-- Based on root.sql schema

CREATE TABLE IF NOT EXISTS rad_order_item (
    rad_order_item_id VARCHAR(36) PRIMARY KEY,
    rad_order_id    VARCHAR(36) NOT NULL,
    proc_id         VARCHAR(36) NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    performed_at    TIMESTAMP,
    performer_staff_id VARCHAR(36),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_roi_ro FOREIGN KEY (rad_order_id) REFERENCES rad_orders(id),
    CONSTRAINT fk_roi_proc FOREIGN KEY (proc_id) REFERENCES rad_procedures(id),
    CONSTRAINT fk_roi_staff FOREIGN KEY (performer_staff_id) REFERENCES staff(staff_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_rad_order_item_order ON rad_order_item(rad_order_id);
CREATE INDEX IF NOT EXISTS idx_rad_order_item_proc ON rad_order_item(proc_id);
CREATE INDEX IF NOT EXISTS idx_rad_order_item_status ON rad_order_item(status);
