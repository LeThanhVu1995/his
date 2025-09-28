-- Pharmacy Dispense Items
-- Based on root.sql schema

CREATE TABLE IF NOT EXISTS dispense_item (
    dispense_item_id VARCHAR(36) PRIMARY KEY,
    dispense_id     VARCHAR(36) NOT NULL,
    prescription_item_id VARCHAR(36) NOT NULL,
    quantity        DECIMAL(12,3) NOT NULL,
    unit            VARCHAR(32),
    batch_id        VARCHAR(36),
    expiry_date     DATE,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_di_disp FOREIGN KEY (dispense_id) REFERENCES dispense(dispense_id),
    CONSTRAINT fk_di_rxi FOREIGN KEY (prescription_item_id) REFERENCES prescription_item(prescription_item_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_dispense_item_dispense ON dispense_item(dispense_id);
CREATE INDEX IF NOT EXISTS idx_dispense_item_prescription_item ON dispense_item(prescription_item_id);
CREATE INDEX IF NOT EXISTS idx_dispense_item_batch ON dispense_item(batch_id);
CREATE INDEX IF NOT EXISTS idx_dispense_item_expiry ON dispense_item(expiry_date);
