-- RIS-PACS Studies
-- PACS-specific study management

CREATE TABLE IF NOT EXISTS imaging_study (
    study_id         VARCHAR(36) PRIMARY KEY,
    imaging_order_id VARCHAR(36) NOT NULL,
    study_uid        VARCHAR(128) NOT NULL UNIQUE, -- DICOM Study UID
    accession_no     VARCHAR(64) UNIQUE NOT NULL,
    modality         VARCHAR(16) NOT NULL, -- CT, MR, US, CR, DR, etc.
    study_date       DATE NOT NULL,
    study_time       TIME,
    study_description TEXT,
    patient_age      VARCHAR(16), -- e.g., "045Y", "012M", "003D"
    patient_sex      VARCHAR(1), -- M, F, O
    referring_physician VARCHAR(255),
    performing_physician VARCHAR(255),
    status           VARCHAR(32) NOT NULL DEFAULT 'SCHEDULED', -- SCHEDULED, IN_PROGRESS, COMPLETED, CANCELLED
    started_at       TIMESTAMP,
    completed_at     TIMESTAMP,
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by       VARCHAR(36),
    updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by       VARCHAR(36),
    deleted_at       TIMESTAMP,
    deleted_by       VARCHAR(36),
    CONSTRAINT fk_is_order FOREIGN KEY (imaging_order_id) REFERENCES imaging_order(imaging_order_id)
);

-- DICOM Series
CREATE TABLE IF NOT EXISTS imaging_series (
    series_id        VARCHAR(36) PRIMARY KEY,
    study_id         VARCHAR(36) NOT NULL,
    series_uid       VARCHAR(128) NOT NULL UNIQUE, -- DICOM Series UID
    series_no        INTEGER NOT NULL,
    modality         VARCHAR(16) NOT NULL,
    series_description TEXT,
    body_part_examined VARCHAR(64),
    protocol_name    VARCHAR(255),
    operator_name    VARCHAR(255),
    performed_procedure_step_start_date DATE,
    performed_procedure_step_start_time TIME,
    number_of_instances INTEGER DEFAULT 0,
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_iser_study FOREIGN KEY (study_id) REFERENCES imaging_study(study_id)
);

-- DICOM Instances/Images
CREATE TABLE IF NOT EXISTS imaging_instance (
    instance_id      VARCHAR(36) PRIMARY KEY,
    series_id        VARCHAR(36) NOT NULL,
    sop_instance_uid VARCHAR(128) NOT NULL UNIQUE, -- DICOM SOP Instance UID
    instance_no      INTEGER NOT NULL,
    content_date     DATE,
    content_time     TIME,
    file_path        TEXT, -- Path to DICOM file in PACS storage
    file_size        BIGINT,
    transfer_syntax_uid VARCHAR(128),
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_iinst_series FOREIGN KEY (series_id) REFERENCES imaging_series(series_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_imaging_study_order ON imaging_study(imaging_order_id);
CREATE INDEX IF NOT EXISTS idx_imaging_study_uid ON imaging_study(study_uid);
CREATE INDEX IF NOT EXISTS idx_imaging_study_accession ON imaging_study(accession_no);
CREATE INDEX IF NOT EXISTS idx_imaging_study_status ON imaging_study(status);
CREATE INDEX IF NOT EXISTS idx_imaging_series_study ON imaging_series(study_id);
CREATE INDEX IF NOT EXISTS idx_imaging_series_uid ON imaging_series(series_uid);
CREATE INDEX IF NOT EXISTS idx_imaging_instance_series ON imaging_instance(series_id);
CREATE INDEX IF NOT EXISTS idx_imaging_instance_uid ON imaging_instance(sop_instance_uid);