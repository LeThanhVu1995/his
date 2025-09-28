-- Pharmacy Prescription Items
-- Based on root.sql schema

CREATE TABLE IF NOT EXISTS prescription_item (
    prescription_item_id VARCHAR(36) PRIMARY KEY,
    prescription_id VARCHAR(36) NOT NULL,
    drug_id         VARCHAR(36) NOT NULL,
    dose_per_take   DECIMAL(12,3),
    dose_unit       VARCHAR(32),
    frequency_text  VARCHAR(64),
    route_code      VARCHAR(64),
    duration_days   INTEGER,
    quantity        DECIMAL(12,3),
    quantity_unit   VARCHAR(32),
    instructions    VARCHAR(1000),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_rxi_rx FOREIGN KEY (prescription_id) REFERENCES prescription(prescription_id),
    CONSTRAINT fk_rxi_drug FOREIGN KEY (drug_id) REFERENCES drug_catalog(drug_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_prescription_item_prescription ON prescription_item(prescription_id);
CREATE INDEX IF NOT EXISTS idx_prescription_item_drug ON prescription_item(drug_id);
