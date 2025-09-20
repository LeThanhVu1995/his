# patient-service (ADT)

Quản lý Patient & Encounter. RBAC theo permission policy từ iam-service.

## Chạy nhanh

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/patient
sqlx migrate run
cargo run -p patient-service
```

## API chính

* `GET  /api/v1/patients` (list/search) — perm: `his.patient.list`
* `POST /api/v1/patients` — perm: `his.patient.create`
* `GET  /api/v1/patients/{id}` — perm: `his.patient.read`
* `PUT  /api/v1/patients/{id}` — perm: `his.patient.update`
* `GET  /api/v1/encounters` — perm: `his.encounter.list`
* `POST /api/v1/encounters` — perm: `his.encounter.create`
* `PUT  /api/v1/encounters/{id}` — perm: `his.encounter.update`
* `PUT  /api/v1/encounters/{id}:close` — perm: `his.encounter.close`

## Swagger

* `/swagger` | `/api-docs/openapi.json`
