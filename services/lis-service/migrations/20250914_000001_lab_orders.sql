-- LIS Service - Lab Orders Migration
-- Aligned with root.sql schema

-- Lab Test Catalog (rename from lab_tests to match root.sql)
CREATE TABLE IF NOT EXISTS lab_test_catalog (
    test_id         VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    specimen_code   VARCHAR(64), -- BLOOD, URINE, etc.
    method_text     VARCHAR(255),
    loinc_code      VARCHAR(64)
);

-- Lab Order (main order table)
CREATE TABLE IF NOT EXISTS lab_order (
    lab_order_id    VARCHAR(36) PRIMARY KEY,
    order_id        VARCHAR(36) NOT NULL,
    collected_at    TIMESTAMP,
    collected_by    VARCHAR(36),
    status          VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    CONSTRAINT fk_labo_order FOREIGN KEY (order_id) REFERENCES clinical_order(order_id),
    CONSTRAINT fk_labo_staff FOREIGN KEY (collected_by) REFERENCES staff(staff_id)
);

-- Lab Order Item (individual tests in an order)
CREATE TABLE IF NOT EXISTS lab_order_item (
    lab_order_item_id VARCHAR(36) PRIMARY KEY,
    lab_order_id    VARCHAR(36) NOT NULL,
    test_id         VARCHAR(36) NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    resulted_at     TIMESTAMP,
    CONSTRAINT fk_loi_labo FOREIGN KEY (lab_order_id) REFERENCES lab_order(lab_order_id),
    CONSTRAINT fk_loi_test FOREIGN KEY (test_id) REFERENCES lab_test_catalog(test_id)
);

-- Lab Result (results for each order item)
CREATE TABLE IF NOT EXISTS lab_result (
    lab_result_id   VARCHAR(36) PRIMARY KEY,
    lab_order_item_id VARCHAR(36) NOT NULL,
    result_status   VARCHAR(32) NOT NULL DEFAULT 'FINAL',
    verified_by     VARCHAR(36),
    verified_at     TIMESTAMP,
    remarks         VARCHAR(1000),
    CONSTRAINT fk_lr_item FOREIGN KEY (lab_order_item_id) REFERENCES lab_order_item(lab_order_item_id),
    CONSTRAINT fk_lr_verifier FOREIGN KEY (verified_by) REFERENCES staff(staff_id)
);

-- Lab Result Value (individual values within a result)
CREATE TABLE IF NOT EXISTS lab_result_value (
    value_id        VARCHAR(36) PRIMARY KEY,
    lab_result_id   VARCHAR(36) NOT NULL,
    analyte_code    VARCHAR(64) NOT NULL,   -- e.g., GLU, WBC
    value_num       DECIMAL(18,6),
    value_text      VARCHAR(255),
    unit            VARCHAR(32),
    ref_low         DECIMAL(18,6),
    ref_high        DECIMAL(18,6),
    CONSTRAINT fk_lrv_result FOREIGN KEY (lab_result_id) REFERENCES lab_result(lab_result_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_lab_order_status ON lab_order(status);
CREATE INDEX IF NOT EXISTS idx_lab_order_item_status ON lab_order_item(status);
CREATE INDEX IF NOT EXISTS idx_lab_result_status ON lab_result(result_status);
CREATE INDEX IF NOT EXISTS idx_lab_result_value_analyte ON lab_result_value(analyte_code);
