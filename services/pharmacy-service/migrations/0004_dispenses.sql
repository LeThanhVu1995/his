-- Pharmacy Dispenses
-- Based on root.sql schema

CREATE TABLE IF NOT EXISTS dispense (
    dispense_id     VARCHAR(36) PRIMARY KEY,
    prescription_id VARCHAR(36) NOT NULL,
    dispensed_by    VARCHAR(36),
    dispensed_at    TIMESTAMP,
    status          VARCHAR(32) NOT NULL DEFAULT 'IN_PROGRESS',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_disp_rx FOREIGN KEY (prescription_id) REFERENCES prescription(prescription_id),
    CONSTRAINT fk_disp_staff FOREIGN KEY (dispensed_by) REFERENCES staff(staff_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_dispense_prescription ON dispense(prescription_id);
CREATE INDEX IF NOT EXISTS idx_dispense_staff ON dispense(dispensed_by);
CREATE INDEX IF NOT EXISTS idx_dispense_status ON dispense(status);
CREATE INDEX IF NOT EXISTS idx_dispense_dispensed ON dispense(dispensed_at);
