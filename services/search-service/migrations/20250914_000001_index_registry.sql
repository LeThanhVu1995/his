-- search-service: Index Registry Table
-- This table stores OpenSearch index configurations for different entity types

CREATE TABLE IF NOT EXISTS index_registry (
    id              UUID PRIMARY KEY,
    index_code      VARCHAR(64) NOT NULL UNIQUE,
    index_name      VARCHAR(255) NOT NULL,
    mapping         JSONB NOT NULL,
    settings        JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for faster lookups by index_code
CREATE INDEX IF NOT EXISTS idx_index_registry_code ON index_registry(index_code);

-- Insert default index configurations
INSERT INTO index_registry (id, index_code, index_name, mapping, settings) VALUES
(
    '00000000-0000-0000-0000-000000000001',
    'patients',
    'his-patients-v1',
    '{
        "properties": {
            "id": {"type": "keyword"},
            "code": {"type": "keyword"},
            "full_name": {
                "type": "text",
                "analyzer": "standard",
                "fields": {
                    "keyword": {"type": "keyword"}
                }
            },
            "date_of_birth": {"type": "date"},
            "gender": {"type": "keyword"},
            "phone_number": {"type": "keyword"},
            "national_id": {"type": "keyword"},
            "address": {
                "type": "text",
                "analyzer": "standard"
            },
            "email": {"type": "keyword"},
            "status": {"type": "keyword"}
        }
    }',
    '{
        "analysis": {
            "analyzer": {
                "standard": {
                    "type": "standard"
                }
            }
        }
    }'
),
(
    '00000000-0000-0000-0000-000000000002',
    'encounters',
    'his-encounters-v1',
    '{
        "properties": {
            "id": {"type": "keyword"},
            "patient_id": {"type": "keyword"},
            "encounter_id": {"type": "keyword"},
            "type_code": {"type": "keyword"},
            "status": {"type": "keyword"},
            "start_time": {"type": "date"},
            "end_time": {"type": "date"},
            "department_name": {
                "type": "text",
                "analyzer": "standard"
            },
            "room_name": {
                "type": "text",
                "analyzer": "standard"
            },
            "attending_staff": {
                "type": "text",
                "analyzer": "standard"
            }
        }
    }',
    '{
        "analysis": {
            "analyzer": {
                "standard": {
                    "type": "standard"
                }
            }
        }
    }'
),
(
    '00000000-0000-0000-0000-000000000003',
    'orders',
    'his-orders-v1',
    '{
        "properties": {
            "id": {"type": "keyword"},
            "order_id": {"type": "keyword"},
            "patient_id": {"type": "keyword"},
            "encounter_id": {"type": "keyword"},
            "order_type": {"type": "keyword"},
            "status": {"type": "keyword"},
            "priority_code": {"type": "keyword"},
            "ordered_at": {"type": "date"},
            "remarks": {
                "type": "text",
                "analyzer": "standard"
            }
        }
    }',
    '{
        "analysis": {
            "analyzer": {
                "standard": {
                    "type": "standard"
                }
            }
        }
    }'
),
(
    '00000000-0000-0000-0000-000000000004',
    'documents',
    'his-documents-v1',
    '{
        "properties": {
            "id": {"type": "keyword"},
            "doc_id": {"type": "keyword"},
            "file_name": {
                "type": "text",
                "analyzer": "standard"
            },
            "mime_type": {"type": "keyword"},
            "entity_name": {"type": "keyword"},
            "entity_id": {"type": "keyword"},
            "uploaded_at": {"type": "date"},
            "uploaded_by": {"type": "keyword"},
            "note": {
                "type": "text",
                "analyzer": "standard"
            }
        }
    }',
    '{
        "analysis": {
            "analyzer": {
                "standard": {
                    "type": "standard"
                }
            }
        }
    }'
)
ON CONFLICT (index_code) DO NOTHING;
