-- Price Lists and Price Items
CREATE TABLE IF NOT EXISTS price_list (
    price_list_id   UUID PRIMARY KEY,
    facility_id     UUID NOT NULL,
    code            VARCHAR(64) NOT NULL,
    name            VARCHAR(255) NOT NULL,
    currency        VARCHAR(16) NOT NULL,
    valid_from      DATE,
    valid_to        DATE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (facility_id, code)
);

CREATE TABLE IF NOT EXISTS price_item (
    price_item_id   UUID PRIMARY KEY,
    price_list_id   UUID NOT NULL REFERENCES price_list(price_list_id),
    service_code    VARCHAR(64) NOT NULL,
    description     VARCHAR(255),
    unit_price      DECIMAL(18,2) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (price_list_id, service_code)
);

-- Charges (for tracking individual charges before invoicing)
CREATE TABLE IF NOT EXISTS bill_charge (
    charge_id       UUID PRIMARY KEY,
    encounter_id    UUID NOT NULL,
    patient_id      UUID NOT NULL,
    service_code    VARCHAR(64) NOT NULL,
    description     VARCHAR(255),
    qty             DECIMAL(18,6) NOT NULL DEFAULT 1,
    unit_price      DECIMAL(18,2) NOT NULL,
    amount          DECIMAL(18,2) NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'PENDING', -- PENDING, INVOICED, CANCELLED
    charged_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_charge_encounter ON bill_charge(encounter_id);
CREATE INDEX IF NOT EXISTS idx_charge_patient ON bill_charge(patient_id);
CREATE INDEX IF NOT EXISTS idx_charge_status ON bill_charge(status);
