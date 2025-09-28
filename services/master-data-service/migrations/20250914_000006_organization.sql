-- Organization structure (hospitals, facilities) aligned with root.sql

CREATE TABLE IF NOT EXISTS org_hospital (
    hospital_id     VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64)  NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    status          VARCHAR(32)  NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36)
);

CREATE TABLE IF NOT EXISTS org_facility (
    facility_id     VARCHAR(36) PRIMARY KEY,
    hospital_id     VARCHAR(36) NOT NULL,
    code            VARCHAR(64)  NOT NULL,
    name            VARCHAR(255) NOT NULL,
    address_line1   VARCHAR(255),
    address_line2   VARCHAR(255),
    district        VARCHAR(255),
    city            VARCHAR(255),
    province        VARCHAR(255),
    country         VARCHAR(64),
    postal_code     VARCHAR(32),
    status          VARCHAR(32)  NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_fac_hosp FOREIGN KEY (hospital_id) REFERENCES org_hospital(hospital_id),
    UNIQUE (hospital_id, code)
);

-- Update org_department to reference facility_id
ALTER TABLE org_department ADD COLUMN IF NOT EXISTS facility_id VARCHAR(36);
ALTER TABLE org_department ADD CONSTRAINT IF NOT EXISTS fk_dept_fac
    FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_org_hospital_code ON org_hospital(code);
CREATE INDEX IF NOT EXISTS idx_org_facility_hosp_code ON org_facility(hospital_id, code);
CREATE INDEX IF NOT EXISTS idx_org_department_fac_code ON org_department(facility_id, code);
