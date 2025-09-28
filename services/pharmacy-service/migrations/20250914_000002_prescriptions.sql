-- Pharmacy Prescriptions
-- Based on root.sql schema

CREATE TABLE IF NOT EXISTS prescription (
    prescription_id VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    prescriber_id   VARCHAR(36),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_rx_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_rx_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_rx_staff FOREIGN KEY (prescriber_id) REFERENCES staff(staff_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_prescription_encounter ON prescription(encounter_id);
CREATE INDEX IF NOT EXISTS idx_prescription_patient ON prescription(patient_id);
CREATE INDEX IF NOT EXISTS idx_prescription_prescriber ON prescription(prescriber_id);
CREATE INDEX IF NOT EXISTS idx_prescription_status ON prescription(status);
CREATE INDEX IF NOT EXISTS idx_prescription_created ON prescription(created_at);
