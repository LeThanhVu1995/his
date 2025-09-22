use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::*;

pub struct WarehouseRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> WarehouseRepo<'a> {
    pub async fn create(&self, w: &Warehouse) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO warehouses(id,code,name,type) VALUES($1,$2,$3,$4)"
        )
        .bind(w.id)
        .bind(&w.code)
        .bind(&w.name)
        .bind(&w.r#type)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, name: Option<&str>, r#type: Option<&str>) -> anyhow::Result<Option<Warehouse>> {
        Ok(sqlx::query_as::<_, Warehouse>(
            r#"UPDATE warehouses SET name=COALESCE($2,name), type=COALESCE($3,type), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,type,created_at,updated_at"#
        )
        .bind(id)
        .bind(name)
        .bind(r#type)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn search_paged(&self, q: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Warehouse>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = if let Some(q) = q {
            let like = format!("%{}%", q);
            let r = sqlx::query_as::<_, Warehouse>(
                r#"SELECT id,code,name,type,created_at,updated_at FROM warehouses WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY code OFFSET $2 LIMIT $3"#
            )
            .bind(&like)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM warehouses WHERE code ILIKE $1 OR name ILIKE $1"#
            )
            .bind(&like)
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, Warehouse>(
                r#"SELECT id,code,name,type,created_at,updated_at FROM warehouses ORDER BY code OFFSET $1 LIMIT $2"#
            )
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM warehouses"#
            )
            .fetch_one(self.db)
            .await?;
            (r, t)
        };

        Ok((rows, total))
    }
}

pub struct ItemRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ItemRepo<'a> {
    pub async fn create(&self, i: &Item) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO inv_items(id,code,name,uom,is_med,is_consumable) VALUES($1,$2,$3,$4,$5,$6)"
        )
        .bind(i.id)
        .bind(&i.code)
        .bind(&i.name)
        .bind(&i.uom)
        .bind(i.is_med)
        .bind(i.is_consumable)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, name: Option<&str>, uom: Option<&str>, is_consumable: Option<bool>) -> anyhow::Result<Option<Item>> {
        Ok(sqlx::query_as::<_, Item>(
            r#"UPDATE inv_items SET name=COALESCE($2,name), uom=COALESCE($3,uom), is_consumable=COALESCE($4,is_consumable), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,uom,is_med,is_consumable,created_at,updated_at"#
        )
        .bind(id)
        .bind(name)
        .bind(uom)
        .bind(is_consumable)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn search_paged(&self, q: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Item>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = if let Some(q) = q {
            let like = format!("%{}%", q);
            let r = sqlx::query_as::<_, Item>(
                r#"SELECT id,code,name,uom,is_med,is_consumable,created_at,updated_at FROM inv_items WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY code OFFSET $2 LIMIT $3"#
            )
            .bind(&like)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM inv_items WHERE code ILIKE $1 OR name ILIKE $1"#
            )
            .bind(&like)
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, Item>(
                r#"SELECT id,code,name,uom,is_med,is_consumable,created_at,updated_at FROM inv_items ORDER BY code OFFSET $1 LIMIT $2"#
            )
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM inv_items"#
            )
            .fetch_one(self.db)
            .await?;
            (r, t)
        };

        Ok((rows, total))
    }
}

pub struct LotRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> LotRepo<'a> {
    pub async fn create(&self, l: &Lot) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO inv_lots(id,item_id,lot_no,exp_date) VALUES($1,$2,$3,$4)"
        )
        .bind(l.id)
        .bind(l.item_id)
        .bind(&l.lot_no)
        .bind(l.exp_date)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_paged(&self, item_id: Option<Uuid>, q: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Lot>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = match (item_id, q) {
            (Some(item), Some(q)) => {
                let like = format!("%{}%", q);
                let r = sqlx::query_as::<_, Lot>(
                    r#"SELECT id,item_id,lot_no,exp_date FROM inv_lots WHERE item_id=$1 AND lot_no ILIKE $2 ORDER BY lot_no OFFSET $3 LIMIT $4"#
                )
                .bind(item)
                .bind(&like)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    r#"SELECT COUNT(1) FROM inv_lots WHERE item_id=$1 AND lot_no ILIKE $2"#
                )
                .bind(item)
                .bind(&like)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            (Some(item), None) => {
                let r = sqlx::query_as::<_, Lot>(
                    r#"SELECT id,item_id,lot_no,exp_date FROM inv_lots WHERE item_id=$1 ORDER BY lot_no OFFSET $2 LIMIT $3"#
                )
                .bind(item)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    r#"SELECT COUNT(1) FROM inv_lots WHERE item_id=$1"#
                )
                .bind(item)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, Lot>(
                    r#"SELECT id,item_id,lot_no,exp_date FROM inv_lots ORDER BY lot_no OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    r#"SELECT COUNT(1) FROM inv_lots"#
                )
                .fetch_one(self.db)
                .await?;
                (r, t)
            }
        };

        Ok((rows, total))
    }
}

pub struct StockRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> StockRepo<'a> {
    pub async fn list_paged(&self, warehouse_id: Option<Uuid>, item_id: Option<Uuid>, page: i64, size: i64) -> anyhow::Result<(Vec<Stock>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = match (warehouse_id, item_id) {
            (Some(wh), Some(item)) => {
                let r = sqlx::query_as::<_, Stock>(
                    r#"SELECT warehouse_id,item_id,lot_id,qty FROM inv_stocks WHERE warehouse_id=$1 AND item_id=$2 ORDER BY item_id,lot_id OFFSET $3 LIMIT $4"#
                )
                .bind(wh)
                .bind(item)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    r#"SELECT COUNT(1) FROM inv_stocks WHERE warehouse_id=$1 AND item_id=$2"#
                )
                .bind(wh)
                .bind(item)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            (Some(wh), None) => {
                let r = sqlx::query_as::<_, Stock>(
                    r#"SELECT warehouse_id,item_id,lot_id,qty FROM inv_stocks WHERE warehouse_id=$1 ORDER BY item_id,lot_id OFFSET $2 LIMIT $3"#
                )
                .bind(wh)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    r#"SELECT COUNT(1) FROM inv_stocks WHERE warehouse_id=$1"#
                )
                .bind(wh)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            (None, Some(item)) => {
                let r = sqlx::query_as::<_, Stock>(
                    r#"SELECT warehouse_id,item_id,lot_id,qty FROM inv_stocks WHERE item_id=$1 ORDER BY warehouse_id,lot_id OFFSET $2 LIMIT $3"#
                )
                .bind(item)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    r#"SELECT COUNT(1) FROM inv_stocks WHERE item_id=$1"#
                )
                .bind(item)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, Stock>(
                    r#"SELECT warehouse_id,item_id,lot_id,qty FROM inv_stocks ORDER BY warehouse_id,item_id,lot_id OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    r#"SELECT COUNT(1) FROM inv_stocks"#
                )
                .fetch_one(self.db)
                .await?;
                (r, t)
            }
        };

        Ok((rows, total))
    }
}

pub struct MovementRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> MovementRepo<'a> {
    pub async fn create(&self, m: &Movement, lines: &[MovementLine], tx: &mut sqlx::Transaction<'_, Postgres>) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO inv_movements(id,mv_no,mv_type,src_wh,dst_wh,note,created_by) VALUES($1,$2,$3,$4,$5,$6,$7)"
        )
        .bind(m.id)
        .bind(&m.mv_no)
        .bind(&m.mv_type)
        .bind(m.src_wh)
        .bind(m.dst_wh)
        .bind(&m.note)
        .bind(&m.created_by)
        .execute(tx.as_mut())
        .await?;

        for line in lines {
            sqlx::query(
                "INSERT INTO inv_movement_lines(id,movement_id,item_id,lot_id,qty) VALUES($1,$2,$3,$4,$5)"
            )
            .bind(line.id)
            .bind(line.movement_id)
            .bind(line.item_id)
            .bind(line.lot_id)
            .bind(line.qty)
            .execute(tx.as_mut())
            .await?;
        }

        Ok(())
    }

    pub async fn list_paged(&self, mv_type: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Movement>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = if let Some(t) = mv_type {
            let r = sqlx::query_as::<_, Movement>(
                r#"SELECT id,mv_no,mv_type,src_wh,dst_wh,note,created_by,created_at FROM inv_movements WHERE mv_type=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#
            )
            .bind(t)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM inv_movements WHERE mv_type=$1"#
            )
            .bind(t)
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, Movement>(
                r#"SELECT id,mv_no,mv_type,src_wh,dst_wh,note,created_by,created_at FROM inv_movements ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
            )
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM inv_movements"#
            )
            .fetch_one(self.db)
            .await?;
            (r, t)
        };

        Ok((rows, total))
    }
}
