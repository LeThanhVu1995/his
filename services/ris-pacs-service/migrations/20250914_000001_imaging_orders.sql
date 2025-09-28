-- RIS-PACS Imaging Orders
-- Based on root.sql rad_order schema

CREATE TABLE IF NOT EXISTS imaging_order (
    imaging_order_id  VARCHAR(36) PRIMARY KEY,
    order_id         VARCHAR(36) NOT NULL,
    patient_id       VARCHAR(36) NOT NULL,
    encounter_id     VARCHAR(36),
    procedure_id     VARCHAR(36) NOT NULL,
    scheduled_at     TIMESTAMP,
    scheduled_room_id VARCHAR(36),
    status           VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    priority         VARCHAR(16) NOT NULL DEFAULT 'ROUTINE',
    reason           TEXT,
    requested_by     VARCHAR(36),
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by       VARCHAR(36),
    updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by       VARCHAR(36),
    deleted_at       TIMESTAMP,
    deleted_by       VARCHAR(36),
    CONSTRAINT fk_io_order FOREIGN KEY (order_id) REFERENCES clinical_order(order_id),
    CONSTRAINT fk_io_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_io_encounter FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_io_procedure FOREIGN KEY (procedure_id) REFERENCES rad_procedure_catalog(proc_id),
    CONSTRAINT fk_io_room FOREIGN KEY (scheduled_room_id) REFERENCES org_room(room_id),
    CONSTRAINT fk_io_requested_by FOREIGN KEY (requested_by) REFERENCES staff(staff_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_imaging_order_patient ON imaging_order(patient_id);
CREATE INDEX IF NOT EXISTS idx_imaging_order_status ON imaging_order(status);
CREATE INDEX IF NOT EXISTS idx_imaging_order_scheduled ON imaging_order(scheduled_at);
CREATE INDEX IF NOT EXISTS idx_imaging_order_procedure ON imaging_order(procedure_id);