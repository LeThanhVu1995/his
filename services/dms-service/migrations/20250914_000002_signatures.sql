CREATE TABLE IF NOT EXISTS dms_signatures (
  id UUID PRIMARY KEY,
  object_id UUID NOT NULL REFERENCES dms_objects(id) ON DELETE CASCADE,
  signer_id UUID,
  signer_name VARCHAR(128),
  signature_alg VARCHAR(32),   -- e.g., PAdES, CAdES, RSA-SHA256
  signature_b64 TEXT NOT NULL, -- chữ ký (CMS/PKCS7) base64 hoặc detached
  signed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  note TEXT
);
CREATE INDEX IF NOT EXISTS idx_dms_sign_object ON dms_signatures(object_id);
