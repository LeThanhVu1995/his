-- EMR Vital Signs and Observations
-- Based on root.sql schema

-- Vital Signs Record
CREATE TABLE IF NOT EXISTS vital_sign_record (
    vs_id           VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    measured_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    recorder_staff_id VARCHAR(36),
    note            VARCHAR(1000)
);

-- Vital Signs Items
CREATE TABLE IF NOT EXISTS vital_sign_item (
    vs_item_id      VARCHAR(36) PRIMARY KEY,
    vs_id           VARCHAR(36) NOT NULL,
    code            VARCHAR(64) NOT NULL,   -- e.g., HR, BP_SYS, BP_DIA, RR, SPO2, TEMP
    value_num       DECIMAL(12,3),
    value_text      VARCHAR(255),
    unit            VARCHAR(32)
);

-- Observations (generic clinical measurements)
CREATE TABLE IF NOT EXISTS observation (
    obs_id          VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    code            VARCHAR(64) NOT NULL,  -- LOINC/SNOMED code
    value_num       DECIMAL(18,6),
    value_text      VARCHAR(1000),
    unit            VARCHAR(32),
    taken_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    performer_staff_id VARCHAR(36),
    status          VARCHAR(32) NOT NULL DEFAULT 'FINAL'
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_vital_sign_encounter ON vital_sign_record(encounter_id);
CREATE INDEX IF NOT EXISTS idx_vital_sign_patient ON vital_sign_record(patient_id);
CREATE INDEX IF NOT EXISTS idx_vital_sign_measured ON vital_sign_record(measured_at);
CREATE INDEX IF NOT EXISTS idx_vital_item_vs ON vital_sign_item(vs_id);
CREATE INDEX IF NOT EXISTS idx_vital_item_code ON vital_sign_item(code);
CREATE INDEX IF NOT EXISTS idx_observation_encounter ON observation(encounter_id);
CREATE INDEX IF NOT EXISTS idx_observation_patient ON observation(patient_id);
CREATE INDEX IF NOT EXISTS idx_observation_code ON observation(code);
CREATE INDEX IF NOT EXISTS idx_observation_taken ON observation(taken_at);
