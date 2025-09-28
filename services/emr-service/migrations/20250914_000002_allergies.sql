-- EMR Allergies and Medications
-- Based on root.sql schema

-- Allergy Intolerance
CREATE TABLE IF NOT EXISTS allergy_intolerance (
    allergy_id      VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    substance_code  VARCHAR(64) NOT NULL,
    reaction_text   VARCHAR(1000),
    severity_code   VARCHAR(64),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    recorded_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Medication Statement
CREATE TABLE IF NOT EXISTS medication_statement (
    med_stmt_id     VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    drug_code       VARCHAR(64) NOT NULL,
    drug_name       VARCHAR(255) NOT NULL,
    dose_text       VARCHAR(255),
    frequency_text  VARCHAR(255),
    route_code      VARCHAR(64),
    start_date      DATE,
    end_date        DATE,
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE'
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_allergy_patient ON allergy_intolerance(patient_id);
CREATE INDEX IF NOT EXISTS idx_allergy_substance ON allergy_intolerance(substance_code);
CREATE INDEX IF NOT EXISTS idx_medication_patient ON medication_statement(patient_id);
CREATE INDEX IF NOT EXISTS idx_medication_drug ON medication_statement(drug_code);
