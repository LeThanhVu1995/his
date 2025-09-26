-- Organization structure subset aligned with root.sql (departments, rooms, beds)

CREATE TABLE IF NOT EXISTS org_department (
    department_id   VARCHAR(36) PRIMARY KEY,
    facility_id     VARCHAR(36) NOT NULL,
    code            VARCHAR(64)  NOT NULL,
    name            VARCHAR(255) NOT NULL,
    type_code       VARCHAR(64),
    parent_id       VARCHAR(36),
    status          VARCHAR(32)  NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_dept_parent FOREIGN KEY (parent_id) REFERENCES org_department(department_id),
    UNIQUE (facility_id, code)
);

CREATE TABLE IF NOT EXISTS org_room (
    room_id         VARCHAR(36) PRIMARY KEY,
    department_id   VARCHAR(36) NOT NULL,
    code            VARCHAR(64)  NOT NULL,
    name            VARCHAR(255),
    type_code       VARCHAR(64),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_room_dept FOREIGN KEY (department_id) REFERENCES org_department(department_id),
    UNIQUE (department_id, code)
);

CREATE TABLE IF NOT EXISTS org_bed (
    bed_id          VARCHAR(36) PRIMARY KEY,
    room_id         VARCHAR(36) NOT NULL,
    code            VARCHAR(64)  NOT NULL,
    name            VARCHAR(255),
    status          VARCHAR(32) NOT NULL DEFAULT 'AVAILABLE',
    created_at      TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_bed_room FOREIGN KEY (room_id) REFERENCES org_room(room_id),
    UNIQUE (room_id, code)
);

CREATE INDEX IF NOT EXISTS idx_org_department_fac_code ON org_department(facility_id, code);
CREATE INDEX IF NOT EXISTS idx_org_room_dept_code ON org_room(department_id, code);
CREATE INDEX IF NOT EXISTS idx_org_bed_room_code ON org_bed(room_id, code);
