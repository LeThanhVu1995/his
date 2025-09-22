# inventory-service
Quản lý tồn kho thuốc/vật tư: danh mục hàng (items), lô (lots), kho (warehouses), tồn (stocks), chứng từ nhập/xuất/điều chỉnh (movements). RBAC theo IAM permissions.

## Run
```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/inventory
sqlx migrate run
cargo run -p inventory-service
```

## API & Permissions

* Warehouses: `GET /api/v1/inv/warehouses` (his.inv.wh.list), `POST /api/v1/inv/warehouses:create` (his.inv.wh.create), `PUT /api/v1/inv/warehouses/{id}` (his.inv.wh.update)
* Items: `GET /api/v1/inv/items` (his.inv.item.list), `POST /api/v1/inv/items:create` (his.inv.item.create), `PUT /api/v1/inv/items/{id}` (his.inv.item.update)
* Lots: `GET /api/v1/inv/lots` (his.inv.lot.list), `POST /api/v1/inv/lots:create` (his.inv.lot.create)
* Stocks: `GET /api/v1/inv/stocks` (his.inv.stock.view)
* Movements: `GET /api/v1/inv/movements` (his.inv.move.list), `POST /api/v1/inv/movements:receive` (his.inv.move.receive), `POST /api/v1/inv/movements:issue` (his.inv.move.issue), `POST /api/v1/inv/movements:transfer` (his.inv.move.transfer), `POST /api/v1/inv/movements:adjust` (his.inv.move.adjust)

## Ghi chú

* Chống double-book: `time_slots` có unique (provider_id, starts_at, ends_at) + khi book cập nhật `reserved=TRUE` trong transaction. Reschedule giải phóng slot cũ.
* Đồng bộ hoá lịch: endpoint `slots:generate` sinh slot theo Schedule trong khoảng ngày.
* Có thể mở rộng: **buffer time** giữa slot, trạng thái slot (blocked), linking với **encounter-service** khi check-in.
