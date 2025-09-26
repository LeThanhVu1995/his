-- search-service: Search-specific database indexes
-- These indexes optimize queries used by the search service for better performance

-- Indexes for patient search optimization
CREATE INDEX IF NOT EXISTS idx_patient_search_name ON patient(full_name) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_patient_search_code ON patient(code) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_patient_search_phone ON patient(phone_number) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_patient_search_national_id ON patient(national_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_patient_search_email ON patient(email) WHERE deleted_at IS NULL;

-- Composite index for patient search with common filters
CREATE INDEX IF NOT EXISTS idx_patient_search_composite ON patient(full_name, code, status) WHERE deleted_at IS NULL;

-- Indexes for encounter search optimization
CREATE INDEX IF NOT EXISTS idx_encounter_search_patient ON encounter(patient_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_encounter_search_type ON encounter(type_code) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_encounter_search_status ON encounter(status) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_encounter_search_time ON encounter(start_time, end_time) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_encounter_search_department ON encounter(department_id) WHERE deleted_at IS NULL;

-- Indexes for clinical order search optimization
CREATE INDEX IF NOT EXISTS idx_order_search_patient ON clinical_order(patient_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_order_search_encounter ON clinical_order(encounter_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_order_search_type ON clinical_order(order_type) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_order_search_status ON clinical_order(status) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_order_search_time ON clinical_order(ordered_at) WHERE deleted_at IS NULL;

-- Indexes for document search optimization
CREATE INDEX IF NOT EXISTS idx_doc_search_name ON doc_file(file_name) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_doc_search_mime ON doc_file(mime_type) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_doc_search_time ON doc_file(uploaded_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_doc_search_user ON doc_file(uploaded_by) WHERE deleted_at IS NULL;

-- Indexes for document links
CREATE INDEX IF NOT EXISTS idx_doc_link_entity ON doc_link(entity_name, entity_id);
CREATE INDEX IF NOT EXISTS idx_doc_link_doc ON doc_link(doc_id);

-- Indexes for organization tables used in search joins
CREATE INDEX IF NOT EXISTS idx_department_name ON org_department(name) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_room_name ON org_room(name) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_staff_user ON staff(user_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_user_name ON users(full_name) WHERE deleted_at IS NULL;

-- Full-text search indexes (if supported by PostgreSQL version)
-- These provide better text search capabilities
CREATE INDEX IF NOT EXISTS idx_patient_fts ON patient USING gin(to_tsvector('english',
    COALESCE(full_name, '') || ' ' ||
    COALESCE(code, '') || ' ' ||
    COALESCE(phone_number, '') || ' ' ||
    COALESCE(national_id, '') || ' ' ||
    COALESCE(email, '') || ' ' ||
    COALESCE(address_line1, '') || ' ' ||
    COALESCE(address_line2, '') || ' ' ||
    COALESCE(city, '') || ' ' ||
    COALESCE(province, '')
)) WHERE deleted_at IS NULL;

-- Index for encounter remarks and notes
CREATE INDEX IF NOT EXISTS idx_encounter_remarks_fts ON encounter USING gin(to_tsvector('english',
    COALESCE(remarks, '')
)) WHERE deleted_at IS NULL;

-- Index for order remarks
CREATE INDEX IF NOT EXISTS idx_order_remarks_fts ON clinical_order USING gin(to_tsvector('english',
    COALESCE(remarks, '')
)) WHERE deleted_at IS NULL;

-- Index for document file names and notes
CREATE INDEX IF NOT EXISTS idx_doc_fts ON doc_file USING gin(to_tsvector('english',
    COALESCE(file_name, '') || ' ' ||
    COALESCE(mime_type, '')
)) WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_doc_link_fts ON doc_link USING gin(to_tsvector('english',
    COALESCE(note, '')
));
