# pharmacy-service

Quản lý thuốc (medications), đơn thuốc (prescriptions), cấp phát (dispenses). RBAC theo IAM permissions.

## Run

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/pharmacy
sqlx migrate run
cargo run -p pharmacy-service
```

## API & Permissions

* Medications: `GET /api/v1/medications` (his.pharmacy.med.list), `POST /medications:create` (his.pharmacy.med.create), `PUT /medications/{id}` (his.pharmacy.med.update)
* Prescriptions: `GET /api/v1/prescriptions` (his.pharmacy.presc.list), `POST /prescriptions:create` (his.pharmacy.presc.create), `PUT /prescriptions/{id}` (his.pharmacy.presc.update)
* Dispenses: `GET /api/v1/dispenses` (his.pharmacy.disp.list), `POST /dispenses:create` (his.pharmacy.disp.create), `PUT /dispenses/{id}:finish` (his.pharmacy.disp.finish)

## Swagger

`/swagger` | `/api-docs/openapi.json`
