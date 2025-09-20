CREATE TABLE IF NOT EXISTS invoices (
  id UUID PRIMARY KEY,
  invoice_no VARCHAR(64) UNIQUE NOT NULL,
  patient_id UUID NOT NULL,
  encounter_id UUID,
  subtotal NUMERIC(12,2) NOT NULL DEFAULT 0,
  discount NUMERIC(12,2) NOT NULL DEFAULT 0,
  tax NUMERIC(12,2) NOT NULL DEFAULT 0,
  total NUMERIC(12,2) NOT NULL DEFAULT 0,
  status VARCHAR(16) NOT NULL DEFAULT 'DRAFT', -- DRAFT/ISSUED/PAID/VOID
  note TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_invoices_encounter ON invoices(encounter_id);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);

