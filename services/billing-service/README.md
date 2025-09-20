# billing-service (AR)
Quản lý Charges, Invoices, Payments. RBAC theo permission từ iam-service.

## Chạy nhanh
```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/billing
sqlx migrate run
cargo run -p billing-service
```

## API

* Charges: `GET /api/v1/charges` (his.billing.charge.list), `POST /api/v1/charges:create` (his.billing.charge.create), `PUT /api/v1/charges/{id}` (his.billing.charge.update)
* Invoices: `GET /api/v1/invoices` (his.billing.invoice.list), `POST /api/v1/invoices:create` (his.billing.invoice.create), `PUT /api/v1/invoices/{id}:issue` (his.billing.invoice.issue)
* Payments: `GET /api/v1/payments` (his.billing.payment.list), `POST /api/v1/payments:create` (his.billing.payment.create)

## Swagger

`/swagger` | `/api-docs/openapi.json`
