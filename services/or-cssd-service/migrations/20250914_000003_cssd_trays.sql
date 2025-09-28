-- CSSD (Sterile Services) tables aligned with root.sql

CREATE TABLE IF NOT EXISTS cssd_tray (
    tray_id         VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    description     VARCHAR(1000),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36)
);

CREATE TABLE IF NOT EXISTS cssd_tray_item (
    tray_item_id    VARCHAR(36) PRIMARY KEY,
    tray_id         VARCHAR(36) NOT NULL,
    instrument_code VARCHAR(64) NOT NULL,
    quantity        INTEGER NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_cti_tray FOREIGN KEY (tray_id) REFERENCES cssd_tray(tray_id),
    CONSTRAINT chk_cti_quantity CHECK (quantity > 0)
);

CREATE TABLE IF NOT EXISTS cssd_sterilization_lot (
    lot_id          VARCHAR(36) PRIMARY KEY,
    lot_code        VARCHAR(64) NOT NULL UNIQUE,
    method_code     VARCHAR(64) NOT NULL,   -- STEAM, EO, PLASMA
    started_at      TIMESTAMP NOT NULL,
    completed_at    TIMESTAMP,
    released_by     VARCHAR(36),
    status          VARCHAR(32) NOT NULL DEFAULT 'IN_PROGRESS',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Note: released_by -> staff(staff_id) in IAM service
    CONSTRAINT chk_cssd_method CHECK (method_code IN ('STEAM', 'EO', 'PLASMA')),
    CONSTRAINT chk_cssd_status CHECK (status IN ('IN_PROGRESS', 'COMPLETED', 'RELEASED', 'FAILED'))
);

CREATE TABLE IF NOT EXISTS cssd_lot_item (
    lot_item_id     VARCHAR(36) PRIMARY KEY,
    lot_id          VARCHAR(36) NOT NULL,
    tray_id         VARCHAR(36) NOT NULL,
    expiry_date     DATE,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_cli_lot FOREIGN KEY (lot_id) REFERENCES cssd_sterilization_lot(lot_id),
    CONSTRAINT fk_cli_tray FOREIGN KEY (tray_id) REFERENCES cssd_tray(tray_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_cssd_tray_code ON cssd_tray(code);
CREATE INDEX IF NOT EXISTS idx_cssd_tray_item_tray_id ON cssd_tray_item(tray_id);
CREATE INDEX IF NOT EXISTS idx_cssd_tray_item_instrument_code ON cssd_tray_item(instrument_code);
CREATE INDEX IF NOT EXISTS idx_cssd_sterilization_lot_code ON cssd_sterilization_lot(lot_code);
CREATE INDEX IF NOT EXISTS idx_cssd_sterilization_lot_status ON cssd_sterilization_lot(status);
CREATE INDEX IF NOT EXISTS idx_cssd_sterilization_lot_started_at ON cssd_sterilization_lot(started_at);
CREATE INDEX IF NOT EXISTS idx_cssd_lot_item_lot_id ON cssd_lot_item(lot_id);
CREATE INDEX IF NOT EXISTS idx_cssd_lot_item_tray_id ON cssd_lot_item(tray_id);
CREATE INDEX IF NOT EXISTS idx_cssd_lot_item_expiry_date ON cssd_lot_item(expiry_date);
