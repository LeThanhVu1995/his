-- Seed OpenSearch index registry with default mappings/settings
INSERT INTO index_registry (id, index_code, index_name, mapping, settings)
VALUES
    (gen_random_uuid(), 'patients',   'his-patients-v1',   '{}'::jsonb, '{}'::jsonb),
    (gen_random_uuid(), 'encounters', 'his-encounters-v1', '{}'::jsonb, '{}'::jsonb),
    (gen_random_uuid(), 'orders',     'his-orders-v1',     '{}'::jsonb, '{}'::jsonb),
    (gen_random_uuid(), 'documents',  'his-documents-v1',  '{}'::jsonb, '{}'::jsonb)
ON CONFLICT (index_code) DO NOTHING;

