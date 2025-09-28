-- EMR Clinical Orders
-- Based on root.sql schema

-- Clinical Orders (parent) for Lab/Radiology/Procedure/Medication
CREATE TABLE IF NOT EXISTS clinical_order (
    order_id        VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    order_type      VARCHAR(64) NOT NULL,   -- LAB, RAD, PROC, MED
    status          VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    ordered_by      VARCHAR(36),
    ordered_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    priority_code   VARCHAR(64),
    remarks         VARCHAR(1000)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_clinical_order_encounter ON clinical_order(encounter_id);
CREATE INDEX IF NOT EXISTS idx_clinical_order_patient ON clinical_order(patient_id);
CREATE INDEX IF NOT EXISTS idx_clinical_order_type ON clinical_order(order_type);
CREATE INDEX IF NOT EXISTS idx_clinical_order_status ON clinical_order(status);
CREATE INDEX IF NOT EXISTS idx_clinical_order_ordered ON clinical_order(ordered_at);
