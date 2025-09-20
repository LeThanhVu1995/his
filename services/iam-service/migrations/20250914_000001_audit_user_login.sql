-- Users & roles (minimal) + audit_user_login
CREATE TABLE IF NOT EXISTS iam_users (
id UUID PRIMARY KEY,
username TEXT NOT NULL UNIQUE,
full_name TEXT,
email TEXT,
locked BOOLEAN NOT NULL DEFAULT FALSE,
created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);


CREATE TABLE IF NOT EXISTS iam_roles (
id UUID PRIMARY KEY,
code TEXT NOT NULL UNIQUE,
name TEXT NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);


CREATE TABLE IF NOT EXISTS iam_user_roles (
user_id UUID NOT NULL REFERENCES iam_users(id) ON DELETE CASCADE,
role_id UUID NOT NULL REFERENCES iam_roles(id) ON DELETE CASCADE,
PRIMARY KEY(user_id, role_id)
);


-- Simple seed roles
INSERT INTO iam_roles (id, code, name)
VALUES
(gen_random_uuid(), 'ADMIN', 'Administrator'),
(gen_random_uuid(), 'USER', 'Standard User')
ON CONFLICT (code) DO NOTHING;


-- Audit table for user login events
CREATE TABLE IF NOT EXISTS audit_user_login (
id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
user_id TEXT NOT NULL,
username TEXT,
ip_addr TEXT,
user_agent TEXT,
success BOOLEAN NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);


CREATE INDEX IF NOT EXISTS idx_audit_user_login_userid ON audit_user_login(user_id);
