-- IoT Service Tables - Aligned with root.sql schema

-- IoT Devices
CREATE TABLE IF NOT EXISTS iot_devices (
    id              VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    type            VARCHAR(64) NOT NULL,  -- VITAL_MONITOR, SENSOR, GATEWAY, etc.
    location        VARCHAR(255),
    last_seen       TIMESTAMP,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Vital Signs (from root.sql)
CREATE TABLE IF NOT EXISTS vital_sign_record (
    vs_id           VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    device_id       VARCHAR(36),  -- Link to IoT device
    measured_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    recorder_staff_id VARCHAR(36),
    note            VARCHAR(1000),
    CONSTRAINT fk_vs_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_vs_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_vs_staff FOREIGN KEY (recorder_staff_id) REFERENCES staff(staff_id),
    CONSTRAINT fk_vs_device FOREIGN KEY (device_id) REFERENCES iot_devices(id)
);

CREATE TABLE IF NOT EXISTS vital_sign_item (
    vs_item_id      VARCHAR(36) PRIMARY KEY,
    vs_id           VARCHAR(36) NOT NULL,
    code            VARCHAR(64) NOT NULL,   -- e.g., HR, BP_SYS, BP_DIA, RR, SPO2, TEMP
    value_num       DECIMAL(12,3),
    value_text      VARCHAR(255),
    unit            VARCHAR(32),
    CONSTRAINT fk_vsi_vs FOREIGN KEY (vs_id) REFERENCES vital_sign_record(vs_id)
);

-- Observations (generic clinical measurements from root.sql)
CREATE TABLE IF NOT EXISTS observation (
    obs_id          VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    device_id       VARCHAR(36),  -- Link to IoT device
    code            VARCHAR(64) NOT NULL,  -- LOINC/SNOMED code
    value_num       DECIMAL(18,6),
    value_text      VARCHAR(1000),
    unit            VARCHAR(32),
    taken_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    performer_staff_id VARCHAR(36),
    status          VARCHAR(32) NOT NULL DEFAULT 'FINAL',
    CONSTRAINT fk_obs_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_obs_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_obs_staff FOREIGN KEY (performer_staff_id) REFERENCES staff(staff_id),
    CONSTRAINT fk_obs_device FOREIGN KEY (device_id) REFERENCES iot_devices(id)
);

-- IoT Device Readings (raw sensor data)
CREATE TABLE IF NOT EXISTS iot_device_reading (
    reading_id      VARCHAR(36) PRIMARY KEY,
    device_id       VARCHAR(36) NOT NULL,
    sensor_type     VARCHAR(64) NOT NULL,  -- TEMPERATURE, HUMIDITY, PRESSURE, etc.
    value_num       DECIMAL(18,6),
    value_text      VARCHAR(255),
    unit            VARCHAR(32),
    quality         VARCHAR(32) DEFAULT 'GOOD',  -- GOOD, BAD, UNCERTAIN
    read_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    raw_data        JSONB,
    CONSTRAINT fk_idr_device FOREIGN KEY (device_id) REFERENCES iot_devices(id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_iot_devices_code ON iot_devices(code);
CREATE INDEX IF NOT EXISTS idx_iot_devices_type ON iot_devices(type);
CREATE INDEX IF NOT EXISTS idx_vital_sign_record_patient ON vital_sign_record(patient_id);
CREATE INDEX IF NOT EXISTS idx_vital_sign_record_encounter ON vital_sign_record(encounter_id);
CREATE INDEX IF NOT EXISTS idx_vital_sign_record_measured_at ON vital_sign_record(measured_at);
CREATE INDEX IF NOT EXISTS idx_vital_sign_item_vs ON vital_sign_item(vs_id);
CREATE INDEX IF NOT EXISTS idx_vital_sign_item_code ON vital_sign_item(code);
CREATE INDEX IF NOT EXISTS idx_observation_patient ON observation(patient_id);
CREATE INDEX IF NOT EXISTS idx_observation_encounter ON observation(encounter_id);
CREATE INDEX IF NOT EXISTS idx_observation_code ON observation(code);
CREATE INDEX IF NOT EXISTS idx_observation_taken_at ON observation(taken_at);
CREATE INDEX IF NOT EXISTS idx_iot_device_reading_device ON iot_device_reading(device_id);
CREATE INDEX IF NOT EXISTS idx_iot_device_reading_sensor_type ON iot_device_reading(sensor_type);
CREATE INDEX IF NOT EXISTS idx_iot_device_reading_read_at ON iot_device_reading(read_at);
