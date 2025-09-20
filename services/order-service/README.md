# Order Service (CPOE)

Quản lý Orders & Order Items với RBAC theo permission policy từ iam-service.

## Chạy nhanh

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/order
sqlx migrate run
cargo run -p order-service
```

## API chính

* `GET  /api/v1/orders` — perm: `his.order.list`
* `POST /api/v1/orders:create` — perm: `his.order.create`
* `PUT  /api/v1/orders/{id}` — perm: `his.order.update`
* `GET  /api/v1/orders/{order_id}/items` — perm: `his.order.read`
* `PUT  /api/v1/order-items/{id}` — perm: `his.order.item.update`
* `POST /api/v1/order-items/{id}:result` — perm: `his.order.item.result`

## Swagger

`/swagger` | `/api-docs/openapi.json`

## Environment Variables

```env
SERVICE_NAME=order-service
SERVICE_PORT=8013
DATABASE_URL=postgres://postgres:postgres@localhost:5432/order
IAM_SERVICE_BASE_URL=http://localhost:8001/api/iam
IAM_SERVICE_TOKEN=changeme
KEYCLOAK_ISSUER=http://localhost:8080/realms/his-realm
KEYCLOAK_AUDIENCE=order-api
```

## Features

- ✅ JWT Authentication via Keycloak
- ✅ Permission-based Authorization
- ✅ Order Management (Lab/Imaging/Procedure)
- ✅ Order Items Management
- ✅ Result Submission
- ✅ OpenAPI Documentation
- ✅ Event Publishing (Kafka)
- ✅ IAM Service Integration
