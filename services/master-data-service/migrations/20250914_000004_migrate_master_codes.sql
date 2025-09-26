-- Move existing data from legacy master_codes to normalized lk_code_set/lk_code

-- 1) Create code sets from distinct categories
INSERT INTO lk_code_set (code_set_id, code, name)
SELECT DISTINCT
    gen_random_uuid()::text AS code_set_id,
    category AS code,
    category AS name
FROM master_codes mc
ON CONFLICT (code) DO NOTHING;

-- 2) Insert codes for each category
INSERT INTO lk_code (code_id, code_set_id, code, display, extra_json)
SELECT
    gen_random_uuid()::text AS code_id,
    s.code_set_id,
    mc.code,
    mc.name,
    mc.description
FROM master_codes mc
JOIN lk_code_set s ON s.code = mc.category
ON CONFLICT (code_set_id, code) DO NOTHING;

