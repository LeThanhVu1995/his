-- Blood Bank Crossmatches
CREATE TABLE IF NOT EXISTS bb_crossmatch (
    crossmatch_id   VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    unit_id         VARCHAR(36) NOT NULL,
    performed_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    result_code     VARCHAR(32),
    performer_id    VARCHAR(36),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_bbc_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_bbc_unit FOREIGN KEY (unit_id) REFERENCES bb_blood_unit(unit_id),
    CONSTRAINT fk_bbc_staff FOREIGN KEY (performer_id) REFERENCES staff(staff_id)
);
