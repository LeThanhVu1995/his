-- Blood Bank Donations
CREATE TABLE IF NOT EXISTS bb_donation (
    donation_id     VARCHAR(36) PRIMARY KEY,
    donor_id        VARCHAR(36) NOT NULL,
    collected_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    volume_ml       INTEGER,
    remarks         VARCHAR(1000),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_bbd_donor FOREIGN KEY (donor_id) REFERENCES bb_donor(donor_id)
);
