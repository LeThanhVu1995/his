-- Blood Bank Adverse Events
CREATE TABLE IF NOT EXISTS bb_adverse_event (
    event_id        VARCHAR(36) PRIMARY KEY,
    issue_id        VARCHAR(36) NOT NULL,
    event_time      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    type_code       VARCHAR(64),
    severity_code   VARCHAR(64),
    description     VARCHAR(1000),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_bbae_issue FOREIGN KEY (issue_id) REFERENCES bb_issue(issue_id)
);
