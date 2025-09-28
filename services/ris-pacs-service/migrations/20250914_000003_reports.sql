-- RIS-PACS Reports
-- Based on root.sql rad_result schema

CREATE TABLE IF NOT EXISTS imaging_report (
    report_id        VARCHAR(36) PRIMARY KEY,
    study_id         VARCHAR(36) NOT NULL,
    report_no        VARCHAR(64) UNIQUE NOT NULL,
    status           VARCHAR(32) NOT NULL DEFAULT 'DRAFT', -- DRAFT, PRELIMINARY, FINAL, AMENDED
    report_text      TEXT,
    findings         TEXT,
    impression       TEXT,
    recommendations  TEXT,
    author_id        VARCHAR(36),
    author_name      VARCHAR(255),
    verified_by      VARCHAR(36),
    verified_at      TIMESTAMP,
    finalized_by     VARCHAR(36),
    finalized_at     TIMESTAMP,
    amended_by       VARCHAR(36),
    amended_at       TIMESTAMP,
    amendment_reason TEXT,
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by       VARCHAR(36),
    updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by       VARCHAR(36),
    deleted_at       TIMESTAMP,
    deleted_by       VARCHAR(36),
    CONSTRAINT fk_ir_study FOREIGN KEY (study_id) REFERENCES imaging_study(study_id),
    CONSTRAINT fk_ir_author FOREIGN KEY (author_id) REFERENCES staff(staff_id),
    CONSTRAINT fk_ir_verified_by FOREIGN KEY (verified_by) REFERENCES staff(staff_id),
    CONSTRAINT fk_ir_finalized_by FOREIGN KEY (finalized_by) REFERENCES staff(staff_id),
    CONSTRAINT fk_ir_amended_by FOREIGN KEY (amended_by) REFERENCES staff(staff_id)
);

-- Report Templates
CREATE TABLE IF NOT EXISTS report_template (
    template_id      VARCHAR(36) PRIMARY KEY,
    name             VARCHAR(255) NOT NULL,
    modality         VARCHAR(16) NOT NULL,
    body_part        VARCHAR(64),
    template_text    TEXT NOT NULL,
    is_active        BOOLEAN NOT NULL DEFAULT TRUE,
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by       VARCHAR(36),
    updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by       VARCHAR(36),
    CONSTRAINT fk_rt_created_by FOREIGN KEY (created_by) REFERENCES staff(staff_id),
    CONSTRAINT fk_rt_updated_by FOREIGN KEY (updated_by) REFERENCES staff(staff_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_imaging_report_study ON imaging_report(study_id);
CREATE INDEX IF NOT EXISTS idx_imaging_report_no ON imaging_report(report_no);
CREATE INDEX IF NOT EXISTS idx_imaging_report_status ON imaging_report(status);
CREATE INDEX IF NOT EXISTS idx_imaging_report_author ON imaging_report(author_id);
CREATE INDEX IF NOT EXISTS idx_report_template_modality ON report_template(modality);
CREATE INDEX IF NOT EXISTS idx_report_template_body_part ON report_template(body_part);