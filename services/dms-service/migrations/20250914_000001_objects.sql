CREATE TABLE IF NOT EXISTS dms_objects (
  id UUID PRIMARY KEY,
  bucket VARCHAR(128) NOT NULL,
  object_key TEXT NOT NULL,
  content_type VARCHAR(128),
  size BIGINT,
  sha256 CHAR(64),
  name VARCHAR(255),
  category VARCHAR(64),      -- EMR, LAB, RIS, BILLING, OTHER
  entity_type VARCHAR(64),   -- patient, encounter, order, claim, ...
  entity_id UUID,
  created_by UUID,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_dms_object_entity ON dms_objects(entity_type, entity_id);
