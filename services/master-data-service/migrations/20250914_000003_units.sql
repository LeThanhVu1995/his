-- Inventory UOM subset for normalization alignment

CREATE TABLE IF NOT EXISTS inv_uom (
    uom_id          VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(32) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL
);

-- Optional basic seed (commented)
-- INSERT INTO inv_uom(uom_id, code, name) VALUES
-- ('00000000-0000-0000-0000-000000000001','EA','Each');
