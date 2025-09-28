-- EMR Core Tables - Patient, Episode, Encounter, Clinical Notes
-- Based on root.sql schema

-- Patient Master Data
CREATE TABLE IF NOT EXISTS patient (
    patient_id      VARCHAR(36) PRIMARY KEY,
    hospital_id     VARCHAR(36) NOT NULL,
    code            VARCHAR(64) UNIQUE, -- MRN
    national_id     VARCHAR(64),
    full_name       VARCHAR(255) NOT NULL,
    date_of_birth   DATE,
    gender          VARCHAR(16),
    phone_number    VARCHAR(20),
    email           VARCHAR(100),
    address_line1   VARCHAR(255),
    address_line2   VARCHAR(255),
    district        VARCHAR(255),
    city            VARCHAR(255),
    province        VARCHAR(255),
    country         VARCHAR(64),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36)
);

-- Patient Identifiers (BHYT, Passport, etc.)
CREATE TABLE IF NOT EXISTS patient_identifier (
    patient_identifier_id VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    system_code     VARCHAR(64) NOT NULL, -- e.g., BHYT, PASSPORT, DRIVER_LICENSE
    value           VARCHAR(100) NOT NULL,
    active          CHAR(1) DEFAULT 'Y',
    UNIQUE (system_code, value)
);

-- Patient Contact Information
CREATE TABLE IF NOT EXISTS patient_contact (
    patient_contact_id VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    relation_code   VARCHAR(64),
    name            VARCHAR(255),
    phone_number    VARCHAR(20),
    email           VARCHAR(100),
    address_line1   VARCHAR(255),
    address_line2   VARCHAR(255),
    city            VARCHAR(255),
    country         VARCHAR(64),
    is_primary      CHAR(1) DEFAULT 'N'
);

-- Episode of Care
CREATE TABLE IF NOT EXISTS episode_of_care (
    episode_id      VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    start_date      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_date        TIMESTAMP,
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    reason_text     VARCHAR(1000)
);

-- Patient Encounters
CREATE TABLE IF NOT EXISTS encounter (
    encounter_id    VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    episode_id      VARCHAR(36),
    facility_id     VARCHAR(36) NOT NULL,
    department_id   VARCHAR(36),
    room_id         VARCHAR(36),
    bed_id          VARCHAR(36),
    type_code       VARCHAR(64) NOT NULL,  -- OPD, IPD, ER, TELEMED
    start_time      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time        TIMESTAMP,
    status          VARCHAR(32) NOT NULL DEFAULT 'IN_PROGRESS',
    attending_staff_id VARCHAR(36)
);

-- Clinical Notes
CREATE TABLE IF NOT EXISTS clinical_note (
    note_id         VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    author_staff_id VARCHAR(36),
    category_code   VARCHAR(64),    -- SOAP, DISCHARGE_SUMMARY, NURSE_NOTE
    content_text    TEXT,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Problem List
CREATE TABLE IF NOT EXISTS problem_list (
    problem_id      VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    code            VARCHAR(64),   -- ICD-10 or SNOMED
    description     VARCHAR(1000),
    onset_date      DATE,
    abatement_date  DATE,
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE'
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_patient_hospital ON patient(hospital_id);
CREATE INDEX IF NOT EXISTS idx_patient_code ON patient(code);
CREATE INDEX IF NOT EXISTS idx_patient_national_id ON patient(national_id);
CREATE INDEX IF NOT EXISTS idx_patient_identifier_patient ON patient_identifier(patient_id);
CREATE INDEX IF NOT EXISTS idx_patient_contact_patient ON patient_contact(patient_id);
CREATE INDEX IF NOT EXISTS idx_episode_patient ON episode_of_care(patient_id);
CREATE INDEX IF NOT EXISTS idx_encounter_patient ON encounter(patient_id);
CREATE INDEX IF NOT EXISTS idx_encounter_episode ON encounter(episode_id);
CREATE INDEX IF NOT EXISTS idx_encounter_facility ON encounter(facility_id);
CREATE INDEX IF NOT EXISTS idx_clinical_note_encounter ON clinical_note(encounter_id);
CREATE INDEX IF NOT EXISTS idx_problem_patient ON problem_list(patient_id);
