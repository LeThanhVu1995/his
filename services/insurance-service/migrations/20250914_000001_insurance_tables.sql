-- Insurance Service Tables - Aligned with root.sql schema

-- Insurance Payers
CREATE TABLE IF NOT EXISTS ins_payer (
    payer_id        VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL
);

-- Insurance Policies
CREATE TABLE IF NOT EXISTS ins_policy (
    policy_id       VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    payer_id        VARCHAR(36) NOT NULL,
    policy_no       VARCHAR(64) NOT NULL,
    coverage_json   TEXT,
    valid_from      DATE,
    valid_to        DATE,
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    CONSTRAINT fk_ins_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_ins_payer FOREIGN KEY (payer_id) REFERENCES ins_payer(payer_id),
    UNIQUE (payer_id, policy_no)
);

-- Insurance Claims
CREATE TABLE IF NOT EXISTS ins_claim (
    claim_id        VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    policy_id       VARCHAR(36) NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'DRAFT',
    total_claimed   DECIMAL(18,2) DEFAULT 0,
    total_approved  DECIMAL(18,2) DEFAULT 0,
    submitted_at    TIMESTAMP,
    response_at     TIMESTAMP,
    response_code   VARCHAR(64),
    response_text   VARCHAR(1000),
    signature_id    VARCHAR(36),
    CONSTRAINT fk_claim_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_claim_policy FOREIGN KEY (policy_id) REFERENCES ins_policy(policy_id)
);

-- Insurance Claim Items
CREATE TABLE IF NOT EXISTS ins_claim_item (
    claim_item_id   VARCHAR(36) PRIMARY KEY,
    claim_id        VARCHAR(36) NOT NULL,
    service_code    VARCHAR(64) NOT NULL,
    description     VARCHAR(255),
    qty             DECIMAL(18,6) DEFAULT 1,
    unit_price      DECIMAL(18,2) DEFAULT 0,
    amount          DECIMAL(18,2) DEFAULT 0,
    approved_amount DECIMAL(18,2) DEFAULT 0,
    CONSTRAINT fk_ci_claim FOREIGN KEY (claim_id) REFERENCES ins_claim(claim_id)
);

-- Digital Signatures
CREATE TABLE IF NOT EXISTS digital_signature (
    signature_id    VARCHAR(36) PRIMARY KEY,
    signer_id       VARCHAR(36),      -- user/staff
    algorithm       VARCHAR(64),
    signed_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    signature_b64   TEXT
);

-- Insurance Claim Attachments
CREATE TABLE IF NOT EXISTS ins_claim_attachment (
    attach_id       VARCHAR(36) PRIMARY KEY,
    claim_id        VARCHAR(36) NOT NULL,
    doc_id          VARCHAR(36) NOT NULL,
    CONSTRAINT fk_ica_claim FOREIGN KEY (claim_id) REFERENCES ins_claim(claim_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_ins_policy_patient ON ins_policy(patient_id);
CREATE INDEX IF NOT EXISTS idx_ins_policy_payer ON ins_policy(payer_id);
CREATE INDEX IF NOT EXISTS idx_ins_claim_encounter ON ins_claim(encounter_id);
CREATE INDEX IF NOT EXISTS idx_ins_claim_policy ON ins_claim(policy_id);
CREATE INDEX IF NOT EXISTS idx_ins_claim_status ON ins_claim(status);
CREATE INDEX IF NOT EXISTS idx_ins_claim_item_claim ON ins_claim_item(claim_id);
CREATE INDEX IF NOT EXISTS idx_ins_claim_attachment_claim ON ins_claim_attachment(claim_id);
CREATE INDEX IF NOT EXISTS idx_digital_signature_signer ON digital_signature(signer_id);
