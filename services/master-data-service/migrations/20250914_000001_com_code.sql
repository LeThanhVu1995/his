-- lk_code_set and lk_code per root.sql (normalized master data)

CREATE TABLE IF NOT EXISTS lk_code_set (
    code_set_id     VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64)  NOT NULL,
    name            VARCHAR(255) NOT NULL,
    description     VARCHAR(1000),
    UNIQUE (code)
);

CREATE TABLE IF NOT EXISTS lk_code (
    code_id         VARCHAR(36) PRIMARY KEY,
    code_set_id     VARCHAR(36) NOT NULL,
    code            VARCHAR(64)  NOT NULL,
    display         VARCHAR(255) NOT NULL,
    extra_json      TEXT,
    CONSTRAINT fk_lk_code_set FOREIGN KEY (code_set_id) REFERENCES lk_code_set(code_set_id),
    UNIQUE (code_set_id, code)
);

CREATE INDEX IF NOT EXISTS idx_lk_code_set_code ON lk_code_set(code);
CREATE INDEX IF NOT EXISTS idx_lk_code_codeset_code ON lk_code(code_set_id, code);
