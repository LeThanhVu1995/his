# radiology-service (RIS)

Quản lý thủ thuật (procedures), order CĐHA, study, report. RBAC theo IAM permissions.

## Run

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/ris
sqlx migrate run
cargo run -p radiology-service
```

## API & Permissions

* Procedures: `GET /api/v1/ris/procedures` (his.ris.proc.list), `POST /ris/procedures:create` (his.ris.proc.create), `PUT /ris/procedures/{id}` (his.ris.proc.update)
* Orders: `GET /api/v1/ris/orders` (his.ris.order.list), `POST /ris/orders:create` (his.ris.order.create), `PUT /ris/orders/{id}` (his.ris.order.update)
* Studies: `GET /api/v1/ris/studies` (his.ris.study.list), `POST /ris/studies:create` (his.ris.study.create), `PUT /ris/studies/{id}:progress` (his.ris.study.progress)
* Reports: `GET /api/v1/ris/reports` (his.ris.report.list), `POST /ris/reports:create` (his.ris.report.create), `PUT /ris/reports/{id}:edit|verify|final` (his.ris.report.edit|verify|final)

## Swagger

`/swagger` | `/api-docs/openapi.json`

## Features

- **Procedure Management**: Manage radiology procedures with modality, body part, contrast requirements
- **Order Management**: Create and manage radiology orders with patient and encounter linking
- **Study Management**: Track study execution with start/end progress tracking
- **Report Management**: Complete report workflow from draft to final with verification
- **RBAC Security**: Permission-based access control via IAM service
- **ETag Caching**: HTTP caching support for list endpoints
- **Pagination**: Standardized pagination for all list endpoints
- **OpenAPI Documentation**: Complete Swagger UI integration
