-- Blood Bank Issues
CREATE TABLE IF NOT EXISTS bb_issue (
    issue_id        VARCHAR(36) PRIMARY KEY,
    unit_id         VARCHAR(36) NOT NULL,
    encounter_id    VARCHAR(36) NOT NULL,
    issued_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    issued_by       VARCHAR(36),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_bbi_unit FOREIGN KEY (unit_id) REFERENCES bb_blood_unit(unit_id),
    CONSTRAINT fk_bbi_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_bbi_staff FOREIGN KEY (issued_by) REFERENCES staff(staff_id)
);
