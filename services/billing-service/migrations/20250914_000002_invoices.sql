-- Invoices and Invoice Items
CREATE TABLE IF NOT EXISTS bill_invoice (
    invoice_id      UUID PRIMARY KEY,
    encounter_id    UUID NOT NULL,
    patient_id      UUID NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'OPEN', -- OPEN, PAID, CANCELLED, REFUNDED
    total_amount    DECIMAL(18,2) NOT NULL DEFAULT 0,
    currency        VARCHAR(16) NOT NULL,
    issued_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    due_date        DATE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS bill_invoice_item (
    invoice_item_id UUID PRIMARY KEY,
    invoice_id      UUID NOT NULL REFERENCES bill_invoice(invoice_id) ON DELETE CASCADE,
    service_code    VARCHAR(64) NOT NULL,
    description     VARCHAR(255),
    qty             DECIMAL(18,6) NOT NULL DEFAULT 1,
    unit_price      DECIMAL(18,2) NOT NULL,
    amount          DECIMAL(18,2) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_invoice_encounter ON bill_invoice(encounter_id);
CREATE INDEX IF NOT EXISTS idx_invoice_patient ON bill_invoice(patient_id);
CREATE INDEX IF NOT EXISTS idx_invoice_status ON bill_invoice(status);
CREATE INDEX IF NOT EXISTS idx_invoice_item_invoice ON bill_invoice_item(invoice_id);
