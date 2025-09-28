-- Payments and Payment Allocations
CREATE TABLE IF NOT EXISTS bill_payment (
    payment_id      UUID PRIMARY KEY,
    invoice_id      UUID NOT NULL REFERENCES bill_invoice(invoice_id),
    method_code     VARCHAR(64) NOT NULL, -- CASH, CARD, BANK, INSURANCE
    amount          DECIMAL(18,2) NOT NULL,
    paid_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ref_no          VARCHAR(128),
    status          VARCHAR(32) NOT NULL DEFAULT 'COMPLETED', -- PENDING, COMPLETED, FAILED, REFUNDED
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS bill_payment_allocation (
    allocation_id   UUID PRIMARY KEY,
    payment_id      UUID NOT NULL REFERENCES bill_payment(payment_id) ON DELETE CASCADE,
    invoice_item_id UUID NOT NULL REFERENCES bill_invoice_item(invoice_item_id),
    amount          DECIMAL(18,2) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Refunds
CREATE TABLE IF NOT EXISTS bill_refund (
    refund_id       UUID PRIMARY KEY,
    payment_id      UUID NOT NULL REFERENCES bill_payment(payment_id),
    amount          DECIMAL(18,2) NOT NULL,
    reason          VARCHAR(255),
    refunded_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ref_no          VARCHAR(128),
    status          VARCHAR(32) NOT NULL DEFAULT 'PENDING', -- PENDING, COMPLETED, FAILED
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_payment_invoice ON bill_payment(invoice_id);
CREATE INDEX IF NOT EXISTS idx_payment_status ON bill_payment(status);
CREATE INDEX IF NOT EXISTS idx_payment_allocation_payment ON bill_payment_allocation(payment_id);
CREATE INDEX IF NOT EXISTS idx_refund_payment ON bill_refund(payment_id);
