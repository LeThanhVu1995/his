-- RBAC/ABAC Policies
CREATE TABLE IF NOT EXISTS iam_policies (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code        TEXT NOT NULL UNIQUE,
  description TEXT,
  effect      TEXT NOT NULL CHECK (effect IN ('allow','deny')),
  actions     TEXT[] NOT NULL,
  resources   TEXT[] NOT NULL,
  condition   JSONB,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS iam_role_policies (
  role_id   UUID NOT NULL REFERENCES iam_roles(id) ON DELETE CASCADE,
  policy_id UUID NOT NULL REFERENCES iam_policies(id) ON DELETE CASCADE,
  PRIMARY KEY (role_id, policy_id)
);

CREATE TABLE IF NOT EXISTS iam_user_policies (
  user_id   UUID NOT NULL REFERENCES iam_users(id) ON DELETE CASCADE,
  policy_id UUID NOT NULL REFERENCES iam_policies(id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, policy_id)
);

