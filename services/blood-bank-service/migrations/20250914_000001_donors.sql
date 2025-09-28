-- Blood Bank Donors
CREATE TABLE IF NOT EXISTS bb_donor (
    donor_id        VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) UNIQUE,
    name            VARCHAR(255) NOT NULL,
    date_of_birth   DATE,
    gender          VARCHAR(16),
    blood_group     VARCHAR(8),    -- A+, A-, B+, O+, etc.
    phone           VARCHAR(32),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
