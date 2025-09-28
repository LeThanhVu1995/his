-- Operating Room (OR) tables aligned with root.sql

CREATE TABLE IF NOT EXISTS or_case (
    or_case_id      VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    scheduled_room_id VARCHAR(36),
    scheduled_start TIMESTAMP,
    scheduled_end   TIMESTAMP,
    actual_start    TIMESTAMP,
    actual_end      TIMESTAMP,
    status          VARCHAR(32) NOT NULL DEFAULT 'SCHEDULED',
    procedure_text  VARCHAR(1000),
    surgeon_staff_id VARCHAR(36),
    anesthetist_staff_id VARCHAR(36),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    -- Note: Foreign key constraints reference tables from other services
    -- encounter_id -> encounter(encounter_id) in EMR service
    -- patient_id -> patient(patient_id) in Patient service
    -- scheduled_room_id -> org_room(room_id) in Master Data service
    -- surgeon_staff_id -> staff(staff_id) in IAM service
    -- anesthetist_staff_id -> staff(staff_id) in IAM service
    CONSTRAINT chk_or_status CHECK (status IN ('SCHEDULED', 'IN_PROGRESS', 'COMPLETED', 'CANCELLED'))
);

CREATE TABLE IF NOT EXISTS or_checklist (
    checklist_id    VARCHAR(36) PRIMARY KEY,
    or_case_id      VARCHAR(36) NOT NULL,
    phase_code      VARCHAR(64) NOT NULL,  -- SIGN_IN, TIME_OUT, SIGN_OUT
    item_code       VARCHAR(64) NOT NULL,
    completed       CHAR(1) DEFAULT 'N',
    completed_at    TIMESTAMP,
    completed_by    VARCHAR(36),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_orc_case FOREIGN KEY (or_case_id) REFERENCES or_case(or_case_id),
    -- Note: completed_by -> staff(staff_id) in IAM service
    CONSTRAINT chk_orc_completed CHECK (completed IN ('Y', 'N')),
    CONSTRAINT chk_orc_phase CHECK (phase_code IN ('SIGN_IN', 'TIME_OUT', 'SIGN_OUT'))
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_or_case_encounter_id ON or_case(encounter_id);
CREATE INDEX IF NOT EXISTS idx_or_case_patient_id ON or_case(patient_id);
CREATE INDEX IF NOT EXISTS idx_or_case_scheduled_room_id ON or_case(scheduled_room_id);
CREATE INDEX IF NOT EXISTS idx_or_case_status ON or_case(status);
CREATE INDEX IF NOT EXISTS idx_or_case_scheduled_start ON or_case(scheduled_start);
CREATE INDEX IF NOT EXISTS idx_or_checklist_or_case_id ON or_checklist(or_case_id);
CREATE INDEX IF NOT EXISTS idx_or_checklist_phase_code ON or_checklist(phase_code);
