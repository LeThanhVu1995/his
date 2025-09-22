# scheduling-service
Đặt lịch dựa trên provider/room; schedules → time slots → appointments. RBAC theo IAM permissions.

## Run
```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/scheduling
sqlx migrate run
cargo run -p scheduling-service
```

## API & Permissions

* Providers: `GET /api/v1/appt/providers` (his.appt.provider.list), `POST /appt/providers:create` (his.appt.provider.create), `PUT /appt/providers/{id}` (his.appt.provider.update)
* Rooms: `GET /api/v1/appt/rooms` (his.appt.room.list), `POST /appt/rooms:create` (his.appt.room.create), `PUT /appt/rooms/{id}` (his.appt.room.update)
* Schedules: `GET /api/v1/appt/schedules` (his.appt.schedule.list), `POST /appt/schedules:create` (his.appt.schedule.create), `PUT /appt/schedules/{id}` (his.appt.schedule.update)
* Slots: `GET /api/v1/appt/slots` (his.appt.slot.list), `POST /appt/slots:generate` (his.appt.slot.generate)
* Appointments: `GET /api/v1/appt/appointments` (his.appt.appt.list), `POST /appt/appointments:book` (his.appt.appt.book), `PUT /appt/appointments/{id}:cancel` (his.appt.appt.cancel), `PUT /appt/appointments/{id}:reschedule` (his.appt.appt.reschedule)

## Ghi chú

* Chống double-book: `time_slots` có unique (provider_id, starts_at, ends_at) + khi book cập nhật `reserved=TRUE` trong transaction. Reschedule giải phóng slot cũ.
* Đồng bộ hoá lịch: endpoint `slots:generate` sinh slot theo Schedule trong khoảng ngày.
* Có thể mở rộng: **buffer time** giữa slot, trạng thái slot (blocked), linking với **encounter-service** khi check-in.
