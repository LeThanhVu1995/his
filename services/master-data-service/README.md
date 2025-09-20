# master-data-service

Service quản lý Master Data cho HIS. Tích hợp IAM-service để đăng ký permission và kiểm tra RBAC theo policy.

## Chạy dev
```bash
# 1) Tạo DB & migration
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/master_data
sqlx migrate run

# 2) Chạy service
cargo run -p master-data-service
```

## Biến môi trường chính

* `DATABASE_URL`
* `IAM_SERVICE_BASE_URL`, `IAM_SERVICE_TOKEN`
* `KEYCLOAK_ISSUER`, `KEYCLOAK_JWKS`, `AUTH_AUDIENCE` (nếu verify trực tiếp)

## API chính (ví dụ)

* `GET  /api/v1/master/codes` — cần `his.master.code.list`
* `POST /api/v1/master/codes:create` — cần `his.master.code.create`
* `PUT  /api/v1/master/codes/{id}` — cần `his.master.code.update`
* `DELETE /api/v1/master/codes/{id}` — cần `his.master.code.delete`

## OpenAPI

* Swagger UI: `GET /swagger`
* OpenAPI JSON: `GET /api-docs/openapi.json`

## Seed permission vào IAM-service

Service tự gọi `POST {IAM_SERVICE_BASE_URL}/policies/register` với danh mục permission từ `security/policy.rs`. Bạn có thể kiểm tra log: `permissions registered with IAM`.

## Tích hợp thật với app-auth

* Thay `http/middleware.rs::DemoAuth` bằng middleware của `app-auth` để verify JWT, fetch scopes/permissions từ Keycloak/IAM.
* Chuẩn claim: nhúng `permissions: ["his.master.code.list", ...]` hoặc map roles -> permissions ở IAM, rồi phát hành token tương ứng.
