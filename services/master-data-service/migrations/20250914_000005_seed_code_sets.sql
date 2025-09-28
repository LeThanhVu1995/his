-- Seed common code sets and codes

-- STATUS_GENERIC
INSERT INTO lk_code_set (code_set_id, code, name)
VALUES (gen_random_uuid()::text, 'STATUS_GENERIC', 'Generic Status')
ON CONFLICT (code) DO NOTHING;

INSERT INTO lk_code (code_id, code_set_id, code, display)
SELECT gen_random_uuid()::text, s.code_set_id, v.code, v.display
FROM (VALUES ('ACTIVE','Active'), ('INACTIVE','Inactive')) v(code, display)
JOIN lk_code_set s ON s.code = 'STATUS_GENERIC'
ON CONFLICT (code_set_id, code) DO NOTHING;

-- GENDER
INSERT INTO lk_code_set (code_set_id, code, name)
VALUES (gen_random_uuid()::text, 'GENDER', 'Gender')
ON CONFLICT (code) DO NOTHING;

INSERT INTO lk_code (code_id, code_set_id, code, display)
SELECT gen_random_uuid()::text, s.code_set_id, v.code, v.display
FROM (VALUES ('MALE','Male'), ('FEMALE','Female'), ('OTHER','Other')) v(code, display)
JOIN lk_code_set s ON s.code = 'GENDER'
ON CONFLICT (code_set_id, code) DO NOTHING;

-- ENCOUNTER_TYPE
INSERT INTO lk_code_set (code_set_id, code, name)
VALUES (gen_random_uuid()::text, 'ENCOUNTER_TYPE', 'Encounter Type')
ON CONFLICT (code) DO NOTHING;

INSERT INTO lk_code (code_id, code_set_id, code, display)
SELECT gen_random_uuid()::text, s.code_set_id, v.code, v.display
FROM (VALUES ('OPD','Outpatient'), ('IPD','Inpatient'), ('ER','Emergency')) v(code, display)
JOIN lk_code_set s ON s.code = 'ENCOUNTER_TYPE'
ON CONFLICT (code_set_id, code) DO NOTHING;

-- ORDER_STATUS
INSERT INTO lk_code_set (code_set_id, code, name)
VALUES (gen_random_uuid()::text, 'ORDER_STATUS', 'Order Status')
ON CONFLICT (code) DO NOTHING;

INSERT INTO lk_code (code_id, code_set_id, code, display)
SELECT gen_random_uuid()::text, s.code_set_id, v.code, v.display
FROM (VALUES ('PLACED','Placed'), ('IN_PROGRESS','In Progress'), ('COMPLETED','Completed'), ('CANCELLED','Cancelled')) v(code, display)
JOIN lk_code_set s ON s.code = 'ORDER_STATUS'
ON CONFLICT (code_set_id, code) DO NOTHING;

