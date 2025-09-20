CREATE TABLE IF NOT EXISTS prescription_items (
  id UUID PRIMARY KEY,
  prescription_id UUID NOT NULL REFERENCES prescriptions(id) ON DELETE CASCADE,
  medication_id UUID NOT NULL REFERENCES medications(id),
  dose VARCHAR(64),           -- 1v, 10ml, 500mg
  freq VARCHAR(64),           -- BID, TID, QID, q8h, ...
  duration VARCHAR(64),       -- 5d, 7d
  qty NUMERIC(12,2) NOT NULL DEFAULT 0,
  instruction TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_prescription_items_p ON prescription_items(prescription_id);
