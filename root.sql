-- ============================================================================
-- HIS Pro - Master Database DDL (Consolidated)
-- Version: 2025-08-10
-- Author: ChatGPT (GPT-5 Thinking)
-- Notes:
--  * Cross-DB friendly (PostgreSQL / MySQL / Oracle) where possible.
--  * Prefer VARCHAR over DB-specific ENUMs. Create lookup tables for codes.
--  * Timestamps use CURRENT_TIMESTAMP (Postgres/MySQL). For Oracle, replace
--    with SYSTIMESTAMP. Adjust AUTO-INCREMENT strategies per RDBMS.
--  * All IDs use VARCHAR(36) UUID strings for portability.
--  * Soft delete via deleted_at, deleted_by.
--  * Auditing via created_at/by and updated_at/by.
--  * This file aggregates modules discussed: Identity/RBAC, Organization,
--    Patient/EMR, Scheduling, OR & CSSD, Lab (LIS), Radiology (RIS/PACS meta),
--    Pharmacy, Inventory & Procurement, Blood Bank, Insurance/BHYT e-Claim,
--    Billing, Workflow Engine, Integration (HL7/FHIR + Instruments),
--    Notifications, Document Store, and Shared Lookups.
-- ============================================================================

-- --------------------------------------------------------------------------
-- 0) MIGRATION TRACKING
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS schema_migrations (
    version         VARCHAR(100)    NOT NULL PRIMARY KEY,
    description     VARCHAR(255)    NOT NULL,
    checksum        VARCHAR(128),
    installed_by    VARCHAR(100)    NOT NULL,
    installed_at    TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- --------------------------------------------------------------------------
-- 1) SHARED LOOKUPS & CODE SETS (extensible, replace/seed as needed)
-- --------------------------------------------------------------------------
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

-- Common seeded sets (examples):
--  * STATUS_GENERIC {ACTIVE, INACTIVE}
--  * GENDER {MALE, FEMALE, OTHER, UNKNOWN}
--  * ENCOUNTER_TYPE {OPD, IPD, ER, TELEMED}
--  * VITAL_SIGN_CODE {HR, BP_SYS, BP_DIA, RR, SPO2, TEMP}
--  * APPOINTMENT_STATUS {BOOKED, ARRIVED, NOSHOW, CANCELLED, COMPLETED}
--  * ORDER_STATUS {PLACED, IN_PROGRESS, COMPLETED, CANCELLED}
--  * CLAIM_STATUS {DRAFT, SUBMITTED, ACK, PAID, REJECTED, PARTIAL}
--  * PAYMENT_METHOD {CASH, CARD, BANK, INSURANCE}

-- --------------------------------------------------------------------------
-- 2) ORGANIZATION STRUCTURE (Hospitals, Facilities, Departments, Locations)
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS org_hospital (
    hospital_id     VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64)  NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    status          VARCHAR(32)  NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36)
);

CREATE TABLE IF NOT EXISTS org_facility (
    facility_id     VARCHAR(36) PRIMARY KEY,
    hospital_id     VARCHAR(36) NOT NULL,
    code            VARCHAR(64)  NOT NULL,
    name            VARCHAR(255) NOT NULL,
    address_line1   VARCHAR(255),
    address_line2   VARCHAR(255),
    district        VARCHAR(255),
    city            VARCHAR(255),
    province        VARCHAR(255),
    country         VARCHAR(64),
    postal_code     VARCHAR(32),
    status          VARCHAR(32)  NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_fac_hosp FOREIGN KEY (hospital_id) REFERENCES org_hospital(hospital_id),
    UNIQUE (hospital_id, code)
);

CREATE TABLE IF NOT EXISTS org_department (
    department_id   VARCHAR(36) PRIMARY KEY,
    facility_id     VARCHAR(36) NOT NULL,
    code            VARCHAR(64)  NOT NULL,
    name            VARCHAR(255) NOT NULL,
    type_code       VARCHAR(64),    -- e.g. CLINIC, WARD, OR, LAB, RADIOLOGY
    parent_id       VARCHAR(36),
    status          VARCHAR(32)  NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_dept_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id),
    CONSTRAINT fk_dept_parent FOREIGN KEY (parent_id) REFERENCES org_department(department_id),
    UNIQUE (facility_id, code)
);

CREATE TABLE IF NOT EXISTS org_room (
    room_id         VARCHAR(36) PRIMARY KEY,
    department_id   VARCHAR(36) NOT NULL,
    code            VARCHAR(64)  NOT NULL,
    name            VARCHAR(255),
    type_code       VARCHAR(64),   -- EXAM_ROOM, WARD_ROOM, OR_ROOM, LAB_AREA
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_room_dept FOREIGN KEY (department_id) REFERENCES org_department(department_id),
    UNIQUE (department_id, code)
);

CREATE TABLE IF NOT EXISTS org_bed (
    bed_id          VARCHAR(36) PRIMARY KEY,
    room_id         VARCHAR(36) NOT NULL,
    code            VARCHAR(64)  NOT NULL,
    name            VARCHAR(255),
    status          VARCHAR(32) NOT NULL DEFAULT 'AVAILABLE',
    created_at      TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_bed_room FOREIGN KEY (room_id) REFERENCES org_room(room_id),
    UNIQUE (room_id, code)
);

-- --------------------------------------------------------------------------
-- 3) IDENTITY & RBAC
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS users (
    user_id         VARCHAR(36) PRIMARY KEY,      -- UUID
    username        VARCHAR(100) NOT NULL UNIQUE,
    password_hash   VARCHAR(255) NOT NULL,
    full_name       VARCHAR(255) NOT NULL,
    email           VARCHAR(100) NOT NULL UNIQUE,
    phone_number    VARCHAR(20),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36)
);

CREATE TABLE IF NOT EXISTS roles (
    role_id         VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(100) NOT NULL UNIQUE, -- e.g., SUPER_ADMIN, DOCTOR
    description     VARCHAR(255),
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36)
);

CREATE TABLE IF NOT EXISTS permissions (
    permission_id   VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(150) NOT NULL UNIQUE, -- e.g., PATIENT_READ, PATIENT_WRITE
    description     VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS role_permissions (
    role_id         VARCHAR(36) NOT NULL,
    permission_id   VARCHAR(36) NOT NULL,
    PRIMARY KEY (role_id, permission_id),
    CONSTRAINT fk_rp_role FOREIGN KEY (role_id) REFERENCES roles(role_id),
    CONSTRAINT fk_rp_perm FOREIGN KEY (permission_id) REFERENCES permissions(permission_id)
);

CREATE TABLE IF NOT EXISTS user_roles (
    user_id         VARCHAR(36) NOT NULL,
    role_id         VARCHAR(36) NOT NULL,
    PRIMARY KEY (user_id, role_id),
    CONSTRAINT fk_ur_user FOREIGN KEY (user_id) REFERENCES users(user_id),
    CONSTRAINT fk_ur_role FOREIGN KEY (role_id) REFERENCES roles(role_id)
);

CREATE TABLE IF NOT EXISTS staff (
    staff_id        VARCHAR(36) PRIMARY KEY,
    user_id         VARCHAR(36) NOT NULL,
    hospital_id     VARCHAR(36) NOT NULL,
    facility_id     VARCHAR(36) NOT NULL,
    department_id   VARCHAR(36),
    practitioner_code VARCHAR(64),
    title           VARCHAR(64),      -- Dr., RN, Tech
    specialty_code  VARCHAR(64),
    license_no      VARCHAR(64),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_staff_user FOREIGN KEY (user_id) REFERENCES users(user_id),
    CONSTRAINT fk_staff_hosp FOREIGN KEY (hospital_id) REFERENCES org_hospital(hospital_id),
    CONSTRAINT fk_staff_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id),
    CONSTRAINT fk_staff_dept FOREIGN KEY (department_id) REFERENCES org_department(department_id)
);

-- Optional mapping for multi-department assignment
CREATE TABLE IF NOT EXISTS staff_departments (
    staff_id        VARCHAR(36) NOT NULL,
    department_id   VARCHAR(36) NOT NULL,
    PRIMARY KEY (staff_id, department_id),
    CONSTRAINT fk_sd_staff FOREIGN KEY (staff_id) REFERENCES staff(staff_id),
    CONSTRAINT fk_sd_dept FOREIGN KEY (department_id) REFERENCES org_department(department_id)
);

-- --------------------------------------------------------------------------
-- 4) PATIENT MASTER DATA & EMR CORE
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS patient (
    patient_id      VARCHAR(36) PRIMARY KEY,
    hospital_id     VARCHAR(36) NOT NULL,
    code            VARCHAR(64) UNIQUE, -- MRN
    national_id     VARCHAR(64),
    full_name       VARCHAR(255) NOT NULL,
    date_of_birth   DATE,
    gender          VARCHAR(16),
    phone_number    VARCHAR(20),
    email           VARCHAR(100),
    address_line1   VARCHAR(255),
    address_line2   VARCHAR(255),
    district        VARCHAR(255),
    city            VARCHAR(255),
    province        VARCHAR(255),
    country         VARCHAR(64),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      VARCHAR(36),
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      VARCHAR(36),
    deleted_at      TIMESTAMP,
    deleted_by      VARCHAR(36),
    CONSTRAINT fk_pt_hosp FOREIGN KEY (hospital_id) REFERENCES org_hospital(hospital_id)
);

CREATE TABLE IF NOT EXISTS patient_identifier (
    patient_identifier_id VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    system_code     VARCHAR(64) NOT NULL, -- e.g., BHYT, PASSPORT, DRIVER_LICENSE
    value           VARCHAR(100) NOT NULL,
    active          CHAR(1) DEFAULT 'Y',
    CONSTRAINT fk_pi_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    UNIQUE (system_code, value)
);

CREATE TABLE IF NOT EXISTS patient_contact (
    patient_contact_id VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    relation_code   VARCHAR(64),
    name            VARCHAR(255),
    phone_number    VARCHAR(20),
    email           VARCHAR(100),
    address_line1   VARCHAR(255),
    address_line2   VARCHAR(255),
    city            VARCHAR(255),
    country         VARCHAR(64),
    is_primary      CHAR(1) DEFAULT 'N',
    CONSTRAINT fk_pc_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id)
);

CREATE TABLE IF NOT EXISTS episode_of_care (
    episode_id      VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    start_date      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_date        TIMESTAMP,
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    reason_text     VARCHAR(1000),
    CONSTRAINT fk_ep_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id)
);

CREATE TABLE IF NOT EXISTS encounter (
    encounter_id    VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    episode_id      VARCHAR(36),
    facility_id     VARCHAR(36) NOT NULL,
    department_id   VARCHAR(36),
    room_id         VARCHAR(36),
    bed_id          VARCHAR(36),
    type_code       VARCHAR(64) NOT NULL,  -- OPD, IPD, ER, TELEMED
    start_time      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time        TIMESTAMP,
    status          VARCHAR(32) NOT NULL DEFAULT 'IN_PROGRESS',
    attending_staff_id VARCHAR(36),
    CONSTRAINT fk_enc_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_enc_episode FOREIGN KEY (episode_id) REFERENCES episode_of_care(episode_id),
    CONSTRAINT fk_enc_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id),
    CONSTRAINT fk_enc_dept FOREIGN KEY (department_id) REFERENCES org_department(department_id),
    CONSTRAINT fk_enc_room FOREIGN KEY (room_id) REFERENCES org_room(room_id),
    CONSTRAINT fk_enc_bed FOREIGN KEY (bed_id) REFERENCES org_bed(bed_id),
    CONSTRAINT fk_enc_staff FOREIGN KEY (attending_staff_id) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS clinical_note (
    note_id         VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    author_staff_id VARCHAR(36),
    category_code   VARCHAR(64),    -- SOAP, DISCHARGE_SUMMARY, NURSE_NOTE
    content_text    TEXT,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_note_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_note_author FOREIGN KEY (author_staff_id) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS allergy_intolerance (
    allergy_id      VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    substance_code  VARCHAR(64) NOT NULL,
    reaction_text   VARCHAR(1000),
    severity_code   VARCHAR(64),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    recorded_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_allergy_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id)
);

CREATE TABLE IF NOT EXISTS problem_list (
    problem_id      VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    code            VARCHAR(64),   -- ICD-10 or SNOMED
    description     VARCHAR(1000),
    onset_date      DATE,
    abatement_date  DATE,
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    CONSTRAINT fk_problem_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id)
);

CREATE TABLE IF NOT EXISTS medication_statement (
    med_stmt_id     VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    drug_code       VARCHAR(64) NOT NULL,
    drug_name       VARCHAR(255) NOT NULL,
    dose_text       VARCHAR(255),
    frequency_text  VARCHAR(255),
    route_code      VARCHAR(64),
    start_date      DATE,
    end_date        DATE,
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    CONSTRAINT fk_ms_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id)
);

-- Vital Signs
CREATE TABLE IF NOT EXISTS vital_sign_record (
    vs_id           VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    measured_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    recorder_staff_id VARCHAR(36),
    note            VARCHAR(1000),
    CONSTRAINT fk_vs_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_vs_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_vs_staff FOREIGN KEY (recorder_staff_id) REFERENCES staff(staff_id)
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

-- Observations (generic clinical measurements)
CREATE TABLE IF NOT EXISTS observation (
    obs_id          VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    code            VARCHAR(64) NOT NULL,  -- LOINC/SNOMED code
    value_num       DECIMAL(18,6),
    value_text      VARCHAR(1000),
    unit            VARCHAR(32),
    taken_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    performer_staff_id VARCHAR(36),
    status          VARCHAR(32) NOT NULL DEFAULT 'FINAL',
    CONSTRAINT fk_obs_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_obs_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_obs_staff FOREIGN KEY (performer_staff_id) REFERENCES staff(staff_id)
);

-- Orders (parent) for Lab/Radiology/Procedure/Medication
CREATE TABLE IF NOT EXISTS clinical_order (
    order_id        VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    order_type      VARCHAR(64) NOT NULL,   -- LAB, RAD, PROC, MED
    status          VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    ordered_by      VARCHAR(36),
    ordered_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    priority_code   VARCHAR(64),
    remarks         VARCHAR(1000),
    CONSTRAINT fk_ord_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_ord_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_ord_staff FOREIGN KEY (ordered_by) REFERENCES staff(staff_id)
);

-- --------------------------------------------------------------------------
-- 5) SCHEDULING (Multi-Resource: practitioner, room, equipment)
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS resource_equipment (
    equipment_id    VARCHAR(36) PRIMARY KEY,
    facility_id     VARCHAR(36) NOT NULL,
    department_id   VARCHAR(36),
    code            VARCHAR(64) NOT NULL,
    name            VARCHAR(255) NOT NULL,
    type_code       VARCHAR(64),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    CONSTRAINT fk_eq_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id),
    CONSTRAINT fk_eq_dept FOREIGN KEY (department_id) REFERENCES org_department(department_id),
    UNIQUE (facility_id, code)
);

CREATE TABLE IF NOT EXISTS practitioner_schedule (
    schedule_id     VARCHAR(36) PRIMARY KEY,
    staff_id        VARCHAR(36) NOT NULL,
    day_of_week     SMALLINT NOT NULL,  -- 1..7
    start_time      TIME NOT NULL,
    end_time        TIME NOT NULL,
    location_room_id VARCHAR(36),
    capacity        INTEGER DEFAULT 1,
    CONSTRAINT fk_ps_staff FOREIGN KEY (staff_id) REFERENCES staff(staff_id),
    CONSTRAINT fk_ps_room FOREIGN KEY (location_room_id) REFERENCES org_room(room_id)
);

CREATE TABLE IF NOT EXISTS appointment (
    appointment_id  VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    facility_id     VARCHAR(36) NOT NULL,
    department_id   VARCHAR(36),
    room_id         VARCHAR(36),
    staff_id        VARCHAR(36),
    start_time      TIMESTAMP NOT NULL,
    end_time        TIMESTAMP NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'BOOKED',
    reason_text     VARCHAR(1000),
    CONSTRAINT fk_ap_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_ap_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id),
    CONSTRAINT fk_ap_dept FOREIGN KEY (department_id) REFERENCES org_department(department_id),
    CONSTRAINT fk_ap_room FOREIGN KEY (room_id) REFERENCES org_room(room_id),
    CONSTRAINT fk_ap_staff FOREIGN KEY (staff_id) REFERENCES staff(staff_id)
);

-- --------------------------------------------------------------------------
-- 6) OPERATING ROOM (OR) & CSSD
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS or_case (
    or_case_id      VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    scheduled_room_id VARCHAR(36),
    scheduled_start TIMESTAMP,
    scheduled_end   TIMESTAMP,
    actual_start    TIMESTAMP,
    actual_end      TIMESTAMP,
    status          VARCHAR(32) NOT NULL DEFAULT 'SCHEDULED',
    procedure_text  VARCHAR(1000),
    surgeon_staff_id VARCHAR(36),
    anesthetist_staff_id VARCHAR(36),
    CONSTRAINT fk_or_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_or_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_or_room FOREIGN KEY (scheduled_room_id) REFERENCES org_room(room_id),
    CONSTRAINT fk_or_surgeon FOREIGN KEY (surgeon_staff_id) REFERENCES staff(staff_id),
    CONSTRAINT fk_or_anest FOREIGN KEY (anesthetist_staff_id) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS or_checklist (
    checklist_id    VARCHAR(36) PRIMARY KEY,
    or_case_id      VARCHAR(36) NOT NULL,
    phase_code      VARCHAR(64) NOT NULL,  -- SIGN_IN, TIME_OUT, SIGN_OUT
    item_code       VARCHAR(64) NOT NULL,
    completed       CHAR(1) DEFAULT 'N',
    completed_at    TIMESTAMP,
    completed_by    VARCHAR(36),
    CONSTRAINT fk_orc_case FOREIGN KEY (or_case_id) REFERENCES or_case(or_case_id),
    CONSTRAINT fk_orc_staff FOREIGN KEY (completed_by) REFERENCES staff(staff_id)
);

-- CSSD (Sterile Services)
CREATE TABLE IF NOT EXISTS cssd_tray (
    tray_id         VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    description     VARCHAR(1000)
);

CREATE TABLE IF NOT EXISTS cssd_tray_item (
    tray_item_id    VARCHAR(36) PRIMARY KEY,
    tray_id         VARCHAR(36) NOT NULL,
    instrument_code VARCHAR(64) NOT NULL,
    quantity        INTEGER NOT NULL,
    CONSTRAINT fk_cti_tray FOREIGN KEY (tray_id) REFERENCES cssd_tray(tray_id)
);

CREATE TABLE IF NOT EXISTS cssd_sterilization_lot (
    lot_id          VARCHAR(36) PRIMARY KEY,
    lot_code        VARCHAR(64) NOT NULL UNIQUE,
    method_code     VARCHAR(64) NOT NULL,   -- STEAM, EO, PLASMA
    started_at      TIMESTAMP NOT NULL,
    completed_at    TIMESTAMP,
    released_by     VARCHAR(36),
    CONSTRAINT fk_cssd_release_staff FOREIGN KEY (released_by) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS cssd_lot_item (
    lot_item_id     VARCHAR(36) PRIMARY KEY,
    lot_id          VARCHAR(36) NOT NULL,
    tray_id         VARCHAR(36) NOT NULL,
    expiry_date     DATE,
    CONSTRAINT fk_cli_lot FOREIGN KEY (lot_id) REFERENCES cssd_sterilization_lot(lot_id),
    CONSTRAINT fk_cli_tray FOREIGN KEY (tray_id) REFERENCES cssd_tray(tray_id)
);

-- --------------------------------------------------------------------------
-- 7) LABORATORY (LIS)
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS lab_test_catalog (
    test_id         VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    specimen_code   VARCHAR(64), -- BLOOD, URINE, etc.
    method_text     VARCHAR(255),
    loinc_code      VARCHAR(64)
);

CREATE TABLE IF NOT EXISTS lab_order (
    lab_order_id    VARCHAR(36) PRIMARY KEY,
    order_id        VARCHAR(36) NOT NULL,
    collected_at    TIMESTAMP,
    collected_by    VARCHAR(36),
    status          VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    CONSTRAINT fk_labo_order FOREIGN KEY (order_id) REFERENCES clinical_order(order_id),
    CONSTRAINT fk_labo_staff FOREIGN KEY (collected_by) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS lab_order_item (
    lab_order_item_id VARCHAR(36) PRIMARY KEY,
    lab_order_id    VARCHAR(36) NOT NULL,
    test_id         VARCHAR(36) NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    resulted_at     TIMESTAMP,
    CONSTRAINT fk_loi_labo FOREIGN KEY (lab_order_id) REFERENCES lab_order(lab_order_id),
    CONSTRAINT fk_loi_test FOREIGN KEY (test_id) REFERENCES lab_test_catalog(test_id)
);

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

-- --------------------------------------------------------------------------
-- 8) RADIOLOGY (RIS/PACS metadata)
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS rad_procedure_catalog (
    proc_id         VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    modality_code   VARCHAR(64)  -- CR, DR, CT, MR, US, NM, MG
);

CREATE TABLE IF NOT EXISTS rad_order (
    rad_order_id    VARCHAR(36) PRIMARY KEY,
    order_id        VARCHAR(36) NOT NULL,
    scheduled_at    TIMESTAMP,
    scheduled_room_id VARCHAR(36),
    status          VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    CONSTRAINT fk_ro_order FOREIGN KEY (order_id) REFERENCES clinical_order(order_id),
    CONSTRAINT fk_ro_room FOREIGN KEY (scheduled_room_id) REFERENCES org_room(room_id)
);

CREATE TABLE IF NOT EXISTS rad_order_item (
    rad_order_item_id VARCHAR(36) PRIMARY KEY,
    rad_order_id    VARCHAR(36) NOT NULL,
    proc_id         VARCHAR(36) NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'PLACED',
    performed_at    TIMESTAMP,
    performer_staff_id VARCHAR(36),
    CONSTRAINT fk_roi_ro FOREIGN KEY (rad_order_id) REFERENCES rad_order(rad_order_id),
    CONSTRAINT fk_roi_proc FOREIGN KEY (proc_id) REFERENCES rad_procedure_catalog(proc_id),
    CONSTRAINT fk_roi_staff FOREIGN KEY (performer_staff_id) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS rad_result (
    rad_result_id   VARCHAR(36) PRIMARY KEY,
    rad_order_item_id VARCHAR(36) NOT NULL,
    report_text     TEXT,
    result_status   VARCHAR(32) NOT NULL DEFAULT 'FINAL',
    reported_at     TIMESTAMP,
    reported_by     VARCHAR(36),
    pacs_study_uid  VARCHAR(128),
    CONSTRAINT fk_rr_item FOREIGN KEY (rad_order_item_id) REFERENCES rad_order_item(rad_order_item_id),
    CONSTRAINT fk_rr_reporter FOREIGN KEY (reported_by) REFERENCES staff(staff_id)
);

-- --------------------------------------------------------------------------
-- 9) PHARMACY (Rx) & MEDICATIONS
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS drug_catalog (
    drug_id         VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    generic_name    VARCHAR(255),
    form_code       VARCHAR(64),       -- TAB, CAP, INJ, SYRUP
    strength_text   VARCHAR(64),
    atc_code        VARCHAR(32)
);

CREATE TABLE IF NOT EXISTS prescription (
    prescription_id VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    prescriber_id   VARCHAR(36),
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_rx_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_rx_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_rx_staff FOREIGN KEY (prescriber_id) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS prescription_item (
    prescription_item_id VARCHAR(36) PRIMARY KEY,
    prescription_id VARCHAR(36) NOT NULL,
    drug_id         VARCHAR(36) NOT NULL,
    dose_per_take   DECIMAL(12,3),
    dose_unit       VARCHAR(32),
    frequency_text  VARCHAR(64),
    route_code      VARCHAR(64),
    duration_days   INTEGER,
    quantity        DECIMAL(12,3),
    quantity_unit   VARCHAR(32),
    instructions    VARCHAR(1000),
    CONSTRAINT fk_rxi_rx FOREIGN KEY (prescription_id) REFERENCES prescription(prescription_id),
    CONSTRAINT fk_rxi_drug FOREIGN KEY (drug_id) REFERENCES drug_catalog(drug_id)
);

CREATE TABLE IF NOT EXISTS dispense (
    dispense_id     VARCHAR(36) PRIMARY KEY,
    prescription_id VARCHAR(36) NOT NULL,
    dispensed_by    VARCHAR(36),
    dispensed_at    TIMESTAMP,
    status          VARCHAR(32) NOT NULL DEFAULT 'IN_PROGRESS',
    CONSTRAINT fk_disp_rx FOREIGN KEY (prescription_id) REFERENCES prescription(prescription_id),
    CONSTRAINT fk_disp_staff FOREIGN KEY (dispensed_by) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS dispense_item (
    dispense_item_id VARCHAR(36) PRIMARY KEY,
    dispense_id     VARCHAR(36) NOT NULL,
    prescription_item_id VARCHAR(36) NOT NULL,
    quantity        DECIMAL(12,3) NOT NULL,
    unit            VARCHAR(32),
    batch_id        VARCHAR(36),
    expiry_date     DATE,
    CONSTRAINT fk_di_disp FOREIGN KEY (dispense_id) REFERENCES dispense(dispense_id),
    CONSTRAINT fk_di_rxi FOREIGN KEY (prescription_item_id) REFERENCES prescription_item(prescription_item_id)
);

-- --------------------------------------------------------------------------
-- 10) INVENTORY & PROCUREMENT (shared with Pharmacy & CSSD)
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS inv_uom (
    uom_id          VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(32) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS inv_item (
    item_id         VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    category_code   VARCHAR(64),     -- DRUG, CONSUMABLE, DEVICE, TRAY
    base_uom_id     VARCHAR(36),
    is_lot_tracked  CHAR(1) DEFAULT 'Y',
    is_expirable    CHAR(1) DEFAULT 'Y',
    CONSTRAINT fk_item_uom FOREIGN KEY (base_uom_id) REFERENCES inv_uom(uom_id)
);

CREATE TABLE IF NOT EXISTS inv_item_uom (
    item_uom_id     VARCHAR(36) PRIMARY KEY,
    item_id         VARCHAR(36) NOT NULL,
    uom_id          VARCHAR(36) NOT NULL,
    factor          DECIMAL(18,6) NOT NULL, -- conversion factor from base
    CONSTRAINT fk_iu_item FOREIGN KEY (item_id) REFERENCES inv_item(item_id),
    CONSTRAINT fk_iu_uom FOREIGN KEY (uom_id) REFERENCES inv_uom(uom_id),
    UNIQUE (item_id, uom_id)
);

CREATE TABLE IF NOT EXISTS inv_supplier (
    supplier_id     VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    phone           VARCHAR(32),
    email           VARCHAR(128)
);

CREATE TABLE IF NOT EXISTS inv_warehouse (
    warehouse_id    VARCHAR(36) PRIMARY KEY,
    facility_id     VARCHAR(36) NOT NULL,
    code            VARCHAR(64) NOT NULL,
    name            VARCHAR(255) NOT NULL,
    type_code       VARCHAR(64), -- MAIN, PHARMACY, CSSD, LAB
    CONSTRAINT fk_wh_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id),
    UNIQUE (facility_id, code)
);

CREATE TABLE IF NOT EXISTS inv_batch (
    batch_id        VARCHAR(36) PRIMARY KEY,
    item_id         VARCHAR(36) NOT NULL,
    lot_no          VARCHAR(64),
    expiry_date     DATE,
    supplier_id     VARCHAR(36),
    CONSTRAINT fk_batch_item FOREIGN KEY (item_id) REFERENCES inv_item(item_id),
    CONSTRAINT fk_batch_supplier FOREIGN KEY (supplier_id) REFERENCES inv_supplier(supplier_id)
);

CREATE TABLE IF NOT EXISTS inv_stock (
    stock_id        VARCHAR(36) PRIMARY KEY,
    warehouse_id    VARCHAR(36) NOT NULL,
    item_id         VARCHAR(36) NOT NULL,
    batch_id        VARCHAR(36),
    quantity        DECIMAL(18,6) NOT NULL,
    uom_id          VARCHAR(36) NOT NULL,
    CONSTRAINT fk_stock_wh FOREIGN KEY (warehouse_id) REFERENCES inv_warehouse(warehouse_id),
    CONSTRAINT fk_stock_item FOREIGN KEY (item_id) REFERENCES inv_item(item_id),
    CONSTRAINT fk_stock_batch FOREIGN KEY (batch_id) REFERENCES inv_batch(batch_id),
    CONSTRAINT fk_stock_uom FOREIGN KEY (uom_id) REFERENCES inv_uom(uom_id),
    UNIQUE (warehouse_id, item_id, COALESCE(batch_id,'-'), uom_id)
);

CREATE TABLE IF NOT EXISTS inv_stock_txn (
    txn_id          VARCHAR(36) PRIMARY KEY,
    warehouse_id    VARCHAR(36) NOT NULL,
    item_id         VARCHAR(36) NOT NULL,
    batch_id        VARCHAR(36),
    qty_delta       DECIMAL(18,6) NOT NULL,
    uom_id          VARCHAR(36) NOT NULL,
    reason_code     VARCHAR(64),    -- GRN, ISSUE, ADJUST, RETURN
    ref_entity      VARCHAR(64),    -- e.g., dispense_id, grn_id
    ref_id          VARCHAR(36),
    occurred_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_txn_wh FOREIGN KEY (warehouse_id) REFERENCES inv_warehouse(warehouse_id),
    CONSTRAINT fk_txn_item FOREIGN KEY (item_id) REFERENCES inv_item(item_id),
    CONSTRAINT fk_txn_batch FOREIGN KEY (batch_id) REFERENCES inv_batch(batch_id),
    CONSTRAINT fk_txn_uom FOREIGN KEY (uom_id) REFERENCES inv_uom(uom_id)
);

CREATE TABLE IF NOT EXISTS inv_purchase_order (
    po_id           VARCHAR(36) PRIMARY KEY,
    supplier_id     VARCHAR(36) NOT NULL,
    facility_id     VARCHAR(36) NOT NULL,
    po_no           VARCHAR(64) NOT NULL UNIQUE,
    status          VARCHAR(32) NOT NULL DEFAULT 'DRAFT',
    ordered_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_po_supplier FOREIGN KEY (supplier_id) REFERENCES inv_supplier(supplier_id),
    CONSTRAINT fk_po_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id)
);

CREATE TABLE IF NOT EXISTS inv_po_item (
    po_item_id      VARCHAR(36) PRIMARY KEY,
    po_id           VARCHAR(36) NOT NULL,
    item_id         VARCHAR(36) NOT NULL,
    quantity        DECIMAL(18,6) NOT NULL,
    uom_id          VARCHAR(36) NOT NULL,
    unit_price      DECIMAL(18,6) NOT NULL,
    CONSTRAINT fk_poi_po FOREIGN KEY (po_id) REFERENCES inv_purchase_order(po_id),
    CONSTRAINT fk_poi_item FOREIGN KEY (item_id) REFERENCES inv_item(item_id),
    CONSTRAINT fk_poi_uom FOREIGN KEY (uom_id) REFERENCES inv_uom(uom_id)
);

CREATE TABLE IF NOT EXISTS inv_goods_receipt (
    grn_id          VARCHAR(36) PRIMARY KEY,
    po_id           VARCHAR(36),
    warehouse_id    VARCHAR(36) NOT NULL,
    grn_no          VARCHAR(64) NOT NULL UNIQUE,
    received_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    received_by     VARCHAR(36),
    CONSTRAINT fk_grn_po FOREIGN KEY (po_id) REFERENCES inv_purchase_order(po_id),
    CONSTRAINT fk_grn_wh FOREIGN KEY (warehouse_id) REFERENCES inv_warehouse(warehouse_id),
    CONSTRAINT fk_grn_staff FOREIGN KEY (received_by) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS inv_grn_item (
    grn_item_id     VARCHAR(36) PRIMARY KEY,
    grn_id          VARCHAR(36) NOT NULL,
    item_id         VARCHAR(36) NOT NULL,
    batch_id        VARCHAR(36),
    quantity        DECIMAL(18,6) NOT NULL,
    uom_id          VARCHAR(36) NOT NULL,
    unit_price      DECIMAL(18,6),
    CONSTRAINT fk_gri_grn FOREIGN KEY (grn_id) REFERENCES inv_goods_receipt(grn_id),
    CONSTRAINT fk_gri_item FOREIGN KEY (item_id) REFERENCES inv_item(item_id),
    CONSTRAINT fk_gri_batch FOREIGN KEY (batch_id) REFERENCES inv_batch(batch_id),
    CONSTRAINT fk_gri_uom FOREIGN KEY (uom_id) REFERENCES inv_uom(uom_id)
);

-- --------------------------------------------------------------------------
-- 11) BLOOD BANK
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS bb_donor (
    donor_id        VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) UNIQUE,
    name            VARCHAR(255) NOT NULL,
    date_of_birth   DATE,
    gender          VARCHAR(16),
    blood_group     VARCHAR(8),    -- A+, A-, B+, O+, etc.
    phone           VARCHAR(32)
);

CREATE TABLE IF NOT EXISTS bb_donation (
    donation_id     VARCHAR(36) PRIMARY KEY,
    donor_id        VARCHAR(36) NOT NULL,
    collected_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    volume_ml       INTEGER,
    remarks         VARCHAR(1000),
    CONSTRAINT fk_bbd_donor FOREIGN KEY (donor_id) REFERENCES bb_donor(donor_id)
);

CREATE TABLE IF NOT EXISTS bb_blood_unit (
    unit_id         VARCHAR(36) PRIMARY KEY,
    donation_id     VARCHAR(36) NOT NULL,
    component_code  VARCHAR(32),      -- WB, PRBC, FFP, PLT
    unit_no         VARCHAR(64) UNIQUE,
    blood_group     VARCHAR(8),
    expiry_date     DATE,
    status          VARCHAR(32) NOT NULL DEFAULT 'AVAILABLE',
    CONSTRAINT fk_bbu_donation FOREIGN KEY (donation_id) REFERENCES bb_donation(donation_id)
);

CREATE TABLE IF NOT EXISTS bb_crossmatch (
    crossmatch_id   VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    unit_id         VARCHAR(36) NOT NULL,
    performed_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    result_code     VARCHAR(32),
    performer_id    VARCHAR(36),
    CONSTRAINT fk_bbc_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_bbc_unit FOREIGN KEY (unit_id) REFERENCES bb_blood_unit(unit_id),
    CONSTRAINT fk_bbc_staff FOREIGN KEY (performer_id) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS bb_issue (
    issue_id        VARCHAR(36) PRIMARY KEY,
    unit_id         VARCHAR(36) NOT NULL,
    encounter_id    VARCHAR(36) NOT NULL,
    issued_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    issued_by       VARCHAR(36),
    CONSTRAINT fk_bbi_unit FOREIGN KEY (unit_id) REFERENCES bb_blood_unit(unit_id),
    CONSTRAINT fk_bbi_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_bbi_staff FOREIGN KEY (issued_by) REFERENCES staff(staff_id)
);

CREATE TABLE IF NOT EXISTS bb_adverse_event (
    event_id        VARCHAR(36) PRIMARY KEY,
    issue_id        VARCHAR(36) NOT NULL,
    event_time      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    type_code       VARCHAR(64),
    severity_code   VARCHAR(64),
    description     VARCHAR(1000),
    CONSTRAINT fk_bbae_issue FOREIGN KEY (issue_id) REFERENCES bb_issue(issue_id)
);

-- --------------------------------------------------------------------------
-- 12) INSURANCE / BHYT e-CLAIM
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS ins_payer (
    payer_id        VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS ins_policy (
    policy_id       VARCHAR(36) PRIMARY KEY,
    patient_id      VARCHAR(36) NOT NULL,
    payer_id        VARCHAR(36) NOT NULL,
    policy_no       VARCHAR(64) NOT NULL,
    coverage_json   TEXT,
    valid_from      DATE,
    valid_to        DATE,
    status          VARCHAR(32) NOT NULL DEFAULT 'ACTIVE',
    CONSTRAINT fk_ins_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_ins_payer FOREIGN KEY (payer_id) REFERENCES ins_payer(payer_id),
    UNIQUE (payer_id, policy_no)
);

CREATE TABLE IF NOT EXISTS ins_claim (
    claim_id        VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    policy_id       VARCHAR(36) NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'DRAFT',
    total_claimed   DECIMAL(18,2) DEFAULT 0,
    total_approved  DECIMAL(18,2) DEFAULT 0,
    submitted_at    TIMESTAMP,
    response_at     TIMESTAMP,
    response_code   VARCHAR(64),
    response_text   VARCHAR(1000),
    signature_id    VARCHAR(36),
    CONSTRAINT fk_claim_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_claim_policy FOREIGN KEY (policy_id) REFERENCES ins_policy(policy_id)
);

CREATE TABLE IF NOT EXISTS ins_claim_item (
    claim_item_id   VARCHAR(36) PRIMARY KEY,
    claim_id        VARCHAR(36) NOT NULL,
    service_code    VARCHAR(64) NOT NULL,
    description     VARCHAR(255),
    qty             DECIMAL(18,6) DEFAULT 1,
    unit_price      DECIMAL(18,2) DEFAULT 0,
    amount          DECIMAL(18,2) DEFAULT 0,
    approved_amount DECIMAL(18,2) DEFAULT 0,
    CONSTRAINT fk_ci_claim FOREIGN KEY (claim_id) REFERENCES ins_claim(claim_id)
);

CREATE TABLE IF NOT EXISTS digital_signature (
    signature_id    VARCHAR(36) PRIMARY KEY,
    signer_id       VARCHAR(36),      -- user/staff
    algorithm       VARCHAR(64),
    signed_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    signature_b64   TEXT
);

CREATE TABLE IF NOT EXISTS ins_claim_attachment (
    attach_id       VARCHAR(36) PRIMARY KEY,
    claim_id        VARCHAR(36) NOT NULL,
    doc_id          VARCHAR(36) NOT NULL,
    CONSTRAINT fk_ica_claim FOREIGN KEY (claim_id) REFERENCES ins_claim(claim_id)
);

-- --------------------------------------------------------------------------
-- 13) BILLING (AR)
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS price_list (
    price_list_id   VARCHAR(36) PRIMARY KEY,
    facility_id     VARCHAR(36) NOT NULL,
    code            VARCHAR(64) NOT NULL,
    name            VARCHAR(255) NOT NULL,
    currency        VARCHAR(16) NOT NULL,
    valid_from      DATE,
    valid_to        DATE,
    CONSTRAINT fk_pl_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id),
    UNIQUE (facility_id, code)
);

CREATE TABLE IF NOT EXISTS price_item (
    price_item_id   VARCHAR(36) PRIMARY KEY,
    price_list_id   VARCHAR(36) NOT NULL,
    service_code    VARCHAR(64) NOT NULL,
    description     VARCHAR(255),
    unit_price      DECIMAL(18,2) NOT NULL,
    CONSTRAINT fk_pi_pl FOREIGN KEY (price_list_id) REFERENCES price_list(price_list_id),
    UNIQUE (price_list_id, service_code)
);

CREATE TABLE IF NOT EXISTS bill_invoice (
    invoice_id      VARCHAR(36) PRIMARY KEY,
    encounter_id    VARCHAR(36) NOT NULL,
    patient_id      VARCHAR(36) NOT NULL,
    status          VARCHAR(32) NOT NULL DEFAULT 'OPEN',
    total_amount    DECIMAL(18,2) NOT NULL DEFAULT 0,
    currency        VARCHAR(16) NOT NULL,
    issued_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_inv_enc FOREIGN KEY (encounter_id) REFERENCES encounter(encounter_id),
    CONSTRAINT fk_inv_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id)
);

CREATE TABLE IF NOT EXISTS bill_invoice_item (
    invoice_item_id VARCHAR(36) PRIMARY KEY,
    invoice_id      VARCHAR(36) NOT NULL,
    service_code    VARCHAR(64) NOT NULL,
    description     VARCHAR(255),
    qty             DECIMAL(18,6) NOT NULL DEFAULT 1,
    unit_price      DECIMAL(18,2) NOT NULL,
    amount          DECIMAL(18,2) NOT NULL,
    CONSTRAINT fk_bii_inv FOREIGN KEY (invoice_id) REFERENCES bill_invoice(invoice_id)
);

CREATE TABLE IF NOT EXISTS bill_payment (
    payment_id      VARCHAR(36) PRIMARY KEY,
    invoice_id      VARCHAR(36) NOT NULL,
    method_code     VARCHAR(64) NOT NULL, -- CASH, CARD, BANK, INSURANCE
    amount          DECIMAL(18,2) NOT NULL,
    paid_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ref_no          VARCHAR(128),
    CONSTRAINT fk_bp_inv FOREIGN KEY (invoice_id) REFERENCES bill_invoice(invoice_id)
);

CREATE TABLE IF NOT EXISTS bill_payment_allocation (
    allocation_id   VARCHAR(36) PRIMARY KEY,
    payment_id      VARCHAR(36) NOT NULL,
    invoice_item_id VARCHAR(36) NOT NULL,
    amount          DECIMAL(18,2) NOT NULL,
    CONSTRAINT fk_bpa_pay FOREIGN KEY (payment_id) REFERENCES bill_payment(payment_id),
    CONSTRAINT fk_bpa_item FOREIGN KEY (invoice_item_id) REFERENCES bill_invoice_item(invoice_item_id)
);

-- --------------------------------------------------------------------------
-- 14) WORKFLOW ENGINE (Templates, Steps, Transitions, Task Instances)
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS wf_template (
    wf_template_id  VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE,
    name            VARCHAR(255) NOT NULL,
    domain_entity   VARCHAR(64) NOT NULL,    -- e.g., ENCOUNTER, CLAIM, ORDER
    version_no      INTEGER NOT NULL DEFAULT 1,
    is_active       CHAR(1) DEFAULT 'Y'
);

CREATE TABLE IF NOT EXISTS wf_step (
    wf_step_id      VARCHAR(36) PRIMARY KEY,
    wf_template_id  VARCHAR(36) NOT NULL,
    code            VARCHAR(64) NOT NULL,
    name            VARCHAR(255) NOT NULL,
    step_type       VARCHAR(64) NOT NULL,    -- USER_TASK, SERVICE_TASK, GATEWAY
    sla_minutes     INTEGER,
    is_start        CHAR(1) DEFAULT 'N',
    is_end          CHAR(1) DEFAULT 'N',
    CONSTRAINT fk_ws_tpl FOREIGN KEY (wf_template_id) REFERENCES wf_template(wf_template_id),
    UNIQUE (wf_template_id, code)
);

CREATE TABLE IF NOT EXISTS wf_transition (
    wf_transition_id VARCHAR(36) PRIMARY KEY,
    wf_template_id  VARCHAR(36) NOT NULL,
    from_step_id    VARCHAR(36) NOT NULL,
    to_step_id      VARCHAR(36) NOT NULL,
    event_code      VARCHAR(64) NOT NULL,   -- e.g., SUBMIT, APPROVE, REJECT
    condition_expr  VARCHAR(1000),          -- optional expression
    CONSTRAINT fk_wt_tpl FOREIGN KEY (wf_template_id) REFERENCES wf_template(wf_template_id),
    CONSTRAINT fk_wt_from FOREIGN KEY (from_step_id) REFERENCES wf_step(wf_step_id),
    CONSTRAINT fk_wt_to   FOREIGN KEY (to_step_id)   REFERENCES wf_step(wf_step_id)
);

CREATE TABLE IF NOT EXISTS wf_task_instance (
    wf_task_id      VARCHAR(36) PRIMARY KEY,
    wf_template_id  VARCHAR(36) NOT NULL,
    current_step_id VARCHAR(36) NOT NULL,
    domain_entity   VARCHAR(64) NOT NULL,   -- e.g., ENCOUNTER
    domain_id       VARCHAR(36) NOT NULL,   -- FK value in that domain
    status          VARCHAR(32) NOT NULL DEFAULT 'OPEN',
    started_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at    TIMESTAMP,
    CONSTRAINT fk_wti_tpl FOREIGN KEY (wf_template_id) REFERENCES wf_template(wf_template_id),
    CONSTRAINT fk_wti_step FOREIGN KEY (current_step_id) REFERENCES wf_step(wf_step_id),
    UNIQUE (domain_entity, domain_id)
);

CREATE TABLE IF NOT EXISTS wf_task_assignment (
    assignment_id   VARCHAR(36) PRIMARY KEY,
    wf_task_id      VARCHAR(36) NOT NULL,
    assignee_staff_id VARCHAR(36),
    assignee_role_id  VARCHAR(36),
    assigned_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_wta_task FOREIGN KEY (wf_task_id) REFERENCES wf_task_instance(wf_task_id),
    CONSTRAINT fk_wta_staff FOREIGN KEY (assignee_staff_id) REFERENCES staff(staff_id),
    CONSTRAINT fk_wta_role FOREIGN KEY (assignee_role_id) REFERENCES roles(role_id)
);

-- --------------------------------------------------------------------------
-- 15) INTEGRATION (HL7/FHIR, Instruments) & MESSAGE LOGS
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS int_channel (
    channel_id      VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64) NOT NULL UNIQUE, -- e.g., HL7_ADT, FHIR_PATIENT, LIS_ANALYZER
    name            VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS int_endpoint (
    endpoint_id     VARCHAR(36) PRIMARY KEY,
    channel_id      VARCHAR(36) NOT NULL,
    direction       VARCHAR(16) NOT NULL,  -- INBOUND / OUTBOUND
    config_json     TEXT,
    is_active       CHAR(1) DEFAULT 'Y',
    CONSTRAINT fk_ie_channel FOREIGN KEY (channel_id) REFERENCES int_channel(channel_id)
);

CREATE TABLE IF NOT EXISTS int_message (
    message_id      VARCHAR(36) PRIMARY KEY,
    channel_id      VARCHAR(36) NOT NULL,
    endpoint_id     VARCHAR(36),
    direction       VARCHAR(16) NOT NULL,
    correlation_id  VARCHAR(128),
    payload_format  VARCHAR(32),   -- HL7v2, FHIR, JSON, XML
    payload_text    TEXT,
    status          VARCHAR(32) NOT NULL DEFAULT 'RECEIVED',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    processed_at    TIMESTAMP,
    error_text      VARCHAR(2000),
    CONSTRAINT fk_im_channel FOREIGN KEY (channel_id) REFERENCES int_channel(channel_id),
    CONSTRAINT fk_im_endpoint FOREIGN KEY (endpoint_id) REFERENCES int_endpoint(endpoint_id)
);

CREATE TABLE IF NOT EXISTS int_message_log (
    log_id          VARCHAR(36) PRIMARY KEY,
    message_id      VARCHAR(36) NOT NULL,
    level           VARCHAR(16) NOT NULL,  -- INFO/WARN/ERROR
    event_time      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    detail_text     VARCHAR(2000),
    CONSTRAINT fk_iml_msg FOREIGN KEY (message_id) REFERENCES int_message(message_id)
);

-- --------------------------------------------------------------------------
-- 16) NOTIFICATIONS
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS notification (
    notification_id VARCHAR(36) PRIMARY KEY,
    code            VARCHAR(64),
    title           VARCHAR(255) NOT NULL,
    body            VARCHAR(1000),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS notification_target (
    notification_id VARCHAR(36) NOT NULL,
    user_id         VARCHAR(36) NOT NULL,
    read_at         TIMESTAMP,
    PRIMARY KEY (notification_id, user_id),
    CONSTRAINT fk_nt_notif FOREIGN KEY (notification_id) REFERENCES notification(notification_id),
    CONSTRAINT fk_nt_user FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- --------------------------------------------------------------------------
-- 17) DOCUMENT STORE (metadata only)
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS doc_file (
    doc_id          VARCHAR(36) PRIMARY KEY,
    file_name       VARCHAR(255) NOT NULL,
    mime_type       VARCHAR(128),
    byte_size       BIGINT,
    storage_uri     VARCHAR(1000) NOT NULL, -- e.g., s3://... or file:///...
    uploaded_by     VARCHAR(36),
    uploaded_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_doc_user FOREIGN KEY (uploaded_by) REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS doc_link (
    doc_link_id     VARCHAR(36) PRIMARY KEY,
    doc_id          VARCHAR(36) NOT NULL,
    entity_name     VARCHAR(64)  NOT NULL, -- e.g., ENCOUNTER, INS_CLAIM
    entity_id       VARCHAR(36)  NOT NULL,
    note            VARCHAR(1000),
    CONSTRAINT fk_dlink_doc FOREIGN KEY (doc_id) REFERENCES doc_file(doc_id)
);

-- --------------------------------------------------------------------------
-- 18) AUDIT LOG
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS audit_log (
    audit_id        VARCHAR(36) PRIMARY KEY,
    event_time      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id         VARCHAR(36),
    entity_name     VARCHAR(64) NOT NULL,
    entity_id       VARCHAR(36) NOT NULL,
    action          VARCHAR(32) NOT NULL,   -- CREATE, UPDATE, DELETE
    before_json     TEXT,
    after_json      TEXT,
    ip_address      VARCHAR(64)
);

-- --------------------------------------------------------------------------
-- 19) INDEXES (examples; tune as needed)
-- --------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_patient_name ON patient(full_name);
CREATE INDEX IF NOT EXISTS idx_encounter_patient ON encounter(patient_id);
CREATE INDEX IF NOT EXISTS idx_encounter_status ON encounter(status);
CREATE INDEX IF NOT EXISTS idx_appointment_time ON appointment(start_time, end_time);
CREATE INDEX IF NOT EXISTS idx_clinical_order_status ON clinical_order(status);
CREATE INDEX IF NOT EXISTS idx_lab_order_status ON lab_order(status);
CREATE INDEX IF NOT EXISTS idx_rad_order_status ON rad_order(status);
CREATE INDEX IF NOT EXISTS idx_invoice_status ON bill_invoice(status);
CREATE INDEX IF NOT EXISTS idx_workflow_domain ON wf_task_instance(domain_entity, domain_id);
CREATE INDEX IF NOT EXISTS idx_int_message_status ON int_message(status, created_at);

-- --------------------------------------------------------------------------
-- 20) SEED EXAMPLES (minimal; extend via migration data scripts)
-- --------------------------------------------------------------------------
-- INSERT INTO lk_code_set (code_set_id, code, name) VALUES
-- ('00000000-0000-0000-0000-000000000001','STATUS_GENERIC','Generic Status');
-- INSERT INTO lk_code (code_id, code_set_id, code, display) VALUES
-- ('00000000-0000-0000-0000-000000000101','00000000-0000-0000-0000-000000000001','ACTIVE','Active');
-- ('00000000-0000-0000-0000-000000000102','00000000-0000-0000-0000-000000000001','INACTIVE','Inactive');

-- ===========================================================================
-- END OF FILE
-- ===========================================================================
