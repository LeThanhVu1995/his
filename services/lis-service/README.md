# lis-service (LIS)
Quản lý danh mục xét nghiệm (lab_tests), phiếu lấy mẫu (lab_specimens), kết quả (lab_results + lab_result_values). RBAC theo IAM permissions.

## Chạy nhanh
```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/lab
sqlx migrate run
cargo run -p lis-service
```

## API & Permissions

* Tests: `GET /api/v1/lab/tests` (his.lab.test.list), `POST /lab/tests:create` (his.lab.test.create), `PUT /lab/tests/{id}` (his.lab.test.update)
* Specimens: `GET /api/v1/lab/specimens` (his.lab.specimen.list), `POST /lab/specimens:create` (his.lab.specimen.create), `PUT /lab/specimens/{id}:collect` (his.lab.specimen.collect), `PUT /lab/specimens/{id}:receive` (his.lab.specimen.receive), `PUT /lab/specimens/{id}:reject` (his.lab.specimen.reject)
* Results: `GET /api/v1/lab/results` (his.lab.result.list), `POST /lab/results:create` (his.lab.result.create), `POST /lab/results/{id}:enter` (his.lab.result.enter), `PUT /lab/results/{id}:verify` (his.lab.result.verify), `PUT /lab/results/{id}:release` (his.lab.result.release)

## Swagger

`/swagger` | `/api-docs/openapi.json`
