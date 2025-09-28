-- Blood Bank Units
CREATE TABLE IF NOT EXISTS bb_blood_unit (
    unit_id         VARCHAR(36) PRIMARY KEY,
    donation_id     VARCHAR(36) NOT NULL,
    component_code  VARCHAR(32),      -- WB, PRBC, FFP, PLT
    unit_no         VARCHAR(64) UNIQUE,
    blood_group     VARCHAR(8),
    expiry_date     DATE,
    status          VARCHAR(32) NOT NULL DEFAULT 'AVAILABLE',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_bbu_donation FOREIGN KEY (donation_id) REFERENCES bb_donation(donation_id)
);
