-- Resource Equipment
-- Based on root.sql resource_equipment schema

CREATE TABLE IF NOT EXISTS resource_equipment (
    equipment_id    VARCHAR(36) PRIMARY KEY,
    facility_id     VARCHAR(36) NOT NULL,
    department_id   VARCHAR(36),
    code            VARCHAR(64) NOT NULL,
    name            VARCHAR(255) NOT NULL,
    type_code       VARCHAR(64),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_eq_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id),
    CONSTRAINT fk_eq_dept FOREIGN KEY (department_id) REFERENCES org_department(department_id),
    UNIQUE (facility_id, code)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_resource_equipment_facility ON resource_equipment(facility_id);
CREATE INDEX IF NOT EXISTS idx_resource_equipment_department ON resource_equipment(department_id);
CREATE INDEX IF NOT EXISTS idx_resource_equipment_status ON resource_equipment(status);
CREATE INDEX IF NOT EXISTS idx_resource_equipment_type ON resource_equipment(type_code);