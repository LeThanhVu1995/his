-- Enhanced Appointments
-- Based on root.sql appointment schema with additional fields

-- Update existing appointments table to match root.sql
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS facility_id VARCHAR(36);
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS department_id VARCHAR(36);
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS staff_id VARCHAR(36);
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS end_time TIMESTAMP;
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS reason_text VARCHAR(1000);
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS created_by VARCHAR(36);
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS updated_by VARCHAR(36);
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMP;
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS deleted_by VARCHAR(36);

-- Add foreign key constraints
ALTER TABLE appointments ADD CONSTRAINT IF NOT EXISTS fk_ap_fac FOREIGN KEY (facility_id) REFERENCES org_facility(facility_id);
ALTER TABLE appointments ADD CONSTRAINT IF NOT EXISTS fk_ap_dept FOREIGN KEY (department_id) REFERENCES org_department(department_id);
ALTER TABLE appointments ADD CONSTRAINT IF NOT EXISTS fk_ap_staff FOREIGN KEY (staff_id) REFERENCES staff(staff_id);

-- Add indexes for performance
CREATE INDEX IF NOT EXISTS idx_appointments_facility ON appointments(facility_id);
CREATE INDEX IF NOT EXISTS idx_appointments_department ON appointments(department_id);
CREATE INDEX IF NOT EXISTS idx_appointments_staff ON appointments(staff_id);
CREATE INDEX IF NOT EXISTS idx_appointments_start_time ON appointments(start_time);
CREATE INDEX IF NOT EXISTS idx_appointments_status ON appointments(status);