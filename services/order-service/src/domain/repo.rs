use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use crate::domain::models::{Order, OrderItem};

pub struct OrderRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> OrderRepo<'a> {
    pub async fn create(&self, o: &Order) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO orders (id,patient_id,encounter_id,order_no,order_type,status,priority,ordered_by,note)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)"#,
        )
        .bind(o.id)
        .bind(o.patient_id)
        .bind(o.encounter_id)
        .bind(&o.order_no)
        .bind(&o.order_type)
        .bind(&o.status)
        .bind(&o.priority)
        .bind(&o.ordered_by)
        .bind(&o.note)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(
        &self,
        id: Uuid,
        priority: Option<&str>,
        note: Option<&str>,
        status: Option<&str>,
    ) -> anyhow::Result<Option<Order>> {
        let row = sqlx::query(
            r#"UPDATE orders SET
                 priority=COALESCE($2,priority),
                 note=COALESCE($3,note),
                 status=COALESCE($4,status),
                 updated_at=NOW()
               WHERE id=$1
               RETURNING id,patient_id,encounter_id,order_no,order_type,status,priority,ordered_by,note,created_at,updated_at"#,
        )
        .bind(id)
        .bind(priority)
        .bind(note)
        .bind(status)
        .fetch_optional(self.db)
        .await?;

        if let Some(row) = row {
            Ok(Some(Order {
                id: row.get("id"),
                patient_id: row.get("patient_id"),
                encounter_id: row.get("encounter_id"),
                order_no: row.get("order_no"),
                order_type: row.get("order_type"),
                status: row.get("status"),
                priority: row.get("priority"),
                ordered_by: row.get("ordered_by"),
                note: row.get("note"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Order>> {
        let row = sqlx::query(
            r#"SELECT id,patient_id,encounter_id,order_no,order_type,status,priority,ordered_by,note,created_at,updated_at
               FROM orders WHERE id=$1"#,
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?;

        if let Some(row) = row {
            Ok(Some(Order {
                id: row.get("id"),
                patient_id: row.get("patient_id"),
                encounter_id: row.get("encounter_id"),
                order_no: row.get("order_no"),
                order_type: row.get("order_type"),
                status: row.get("status"),
                priority: row.get("priority"),
                ordered_by: row.get("ordered_by"),
                note: row.get("note"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_paged(
        &self,
        patient_id: Option<Uuid>,
        encounter_id: Option<Uuid>,
        status: Option<&str>,
        page: i64,
        size: i64,
    ) -> anyhow::Result<(Vec<Order>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = match (patient_id, encounter_id, status) {
            (Some(p), Some(e), Some(s)) => {
                let rows = sqlx::query(
                    r#"SELECT id,patient_id,encounter_id,order_no,order_type,status,priority,ordered_by,note,created_at,updated_at
                       FROM orders WHERE patient_id=$1 AND encounter_id=$2 AND status=$3
                       ORDER BY created_at DESC OFFSET $4 LIMIT $5"#,
                )
                .bind(p)
                .bind(e)
                .bind(s)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;

                let total = sqlx::query(
                    "SELECT COUNT(1) FROM orders WHERE patient_id=$1 AND encounter_id=$2 AND status=$3",
                )
                .bind(p)
                .bind(e)
                .bind(s)
                .fetch_one(self.db)
                .await?
                .get::<i64, _>(0);

                (rows, total)
            }
            (Some(p), None, Some(s)) => {
                let rows = sqlx::query(
                    r#"SELECT id,patient_id,encounter_id,order_no,order_type,status,priority,ordered_by,note,created_at,updated_at
                       FROM orders WHERE patient_id=$1 AND status=$2
                       ORDER BY created_at DESC OFFSET $3 LIMIT $4"#,
                )
                .bind(p)
                .bind(s)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;

                let total = sqlx::query(
                    "SELECT COUNT(1) FROM orders WHERE patient_id=$1 AND status=$2",
                )
                .bind(p)
                .bind(s)
                .fetch_one(self.db)
                .await?
                .get::<i64, _>(0);

                (rows, total)
            }
            _ => {
                let rows = sqlx::query(
                    r#"SELECT id,patient_id,encounter_id,order_no,order_type,status,priority,ordered_by,note,created_at,updated_at
                       FROM orders ORDER BY created_at DESC OFFSET $1 LIMIT $2"#,
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;

                let total = sqlx::query("SELECT COUNT(1) FROM orders")
                    .fetch_one(self.db)
                    .await?
                    .get::<i64, _>(0);

                (rows, total)
            }
        };

        let items: Vec<Order> = rows
            .into_iter()
            .map(|row| Order {
                id: row.get("id"),
                patient_id: row.get("patient_id"),
                encounter_id: row.get("encounter_id"),
                order_no: row.get("order_no"),
                order_type: row.get("order_type"),
                status: row.get("status"),
                priority: row.get("priority"),
                ordered_by: row.get("ordered_by"),
                note: row.get("note"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok((items, total))
    }
}

pub struct ItemRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ItemRepo<'a> {
    pub async fn add(&self, it: &OrderItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO order_items (id,order_id,item_code,item_name,quantity,status,result_json)
               VALUES ($1,$2,$3,$4,$5,$6,$7)"#,
        )
        .bind(it.id)
        .bind(it.order_id)
        .bind(&it.item_code)
        .bind(&it.item_name)
        .bind(it.quantity)
        .bind(&it.status)
        .bind(&it.result_json)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        qty: Option<i32>,
        status: Option<&str>,
    ) -> anyhow::Result<Option<OrderItem>> {
        let row = sqlx::query(
            r#"UPDATE order_items SET
                 item_name=COALESCE($2,item_name),
                 quantity=COALESCE($3,quantity),
                 status=COALESCE($4,status),
                 updated_at=NOW()
               WHERE id=$1
               RETURNING id,order_id,item_code,item_name,quantity,status,result_json,created_at,updated_at"#,
        )
        .bind(id)
        .bind(name)
        .bind(qty)
        .bind(status)
        .fetch_optional(self.db)
        .await?;

        if let Some(row) = row {
            Ok(Some(OrderItem {
                id: row.get("id"),
                order_id: row.get("order_id"),
                item_code: row.get("item_code"),
                item_name: row.get("item_name"),
                quantity: row.get("quantity"),
                status: row.get("status"),
                result_json: row.get("result_json"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn submit_result(
        &self,
        id: Uuid,
        result: &serde_json::Value,
        status: Option<&str>,
    ) -> anyhow::Result<Option<OrderItem>> {
        let row = sqlx::query(
            r#"UPDATE order_items SET
                 result_json=$2,
                 status=COALESCE($3,status),
                 updated_at=NOW()
               WHERE id=$1
               RETURNING id,order_id,item_code,item_name,quantity,status,result_json,created_at,updated_at"#,
        )
        .bind(id)
        .bind(result)
        .bind(status)
        .fetch_optional(self.db)
        .await?;

        if let Some(row) = row {
            Ok(Some(OrderItem {
                id: row.get("id"),
                order_id: row.get("order_id"),
                item_code: row.get("item_code"),
                item_name: row.get("item_name"),
                quantity: row.get("quantity"),
                status: row.get("status"),
                result_json: row.get("result_json"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_by_order(&self, order_id: Uuid) -> anyhow::Result<Vec<OrderItem>> {
        let rows = sqlx::query(
            r#"SELECT id,order_id,item_code,item_name,quantity,status,result_json,created_at,updated_at
               FROM order_items WHERE order_id=$1 ORDER BY created_at"#,
        )
        .bind(order_id)
        .fetch_all(self.db)
        .await?;

        let items: Vec<OrderItem> = rows
            .into_iter()
            .map(|row| OrderItem {
                id: row.get("id"),
                order_id: row.get("order_id"),
                item_code: row.get("item_code"),
                item_name: row.get("item_name"),
                quantity: row.get("quantity"),
                status: row.get("status"),
                result_json: row.get("result_json"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(items)
    }
}
