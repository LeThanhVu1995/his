-- Pharmacy Drug Catalog
-- Based on root.sql schema

CREATE TABLE IF NOT EXISTS drug_catalog (
    drug_id         VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    generic_name    VARCHAR(255),
    form_code       VARCHAR(64),       -- TAB, CAP, INJ, SYRUP
    strength_text   VARCHAR(64),
    atc_code        VARCHAR(32),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_drug_catalog_code ON drug_catalog(code);
CREATE INDEX IF NOT EXISTS idx_drug_catalog_name ON drug_catalog(name);
CREATE INDEX IF NOT EXISTS idx_drug_catalog_atc ON drug_catalog(atc_code);
