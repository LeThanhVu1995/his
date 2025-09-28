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
// UOM Repository
pub struct UomRepo<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> UomRepo<'a> {
    pub async fn create(&self, u: &crate::domain::models::Uom) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO inv_uom(id,code,name,created_at,updated_at) VALUES($1,$2,$3,$4,$5)"#
        )
        .bind(&u.id)
        .bind(&u.code)
        .bind(&u.name)
        .bind(&u.created_at)
        .bind(&u.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_paged(&self, q: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<crate::domain::models::Uom>, i64)> {
        let offset = (page - 1) * size;
        let (rows, total) = if let Some(query) = q {
            let r = sqlx::query_as::<_, crate::domain::models::Uom>(
                r#"SELECT * FROM inv_uom WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY code LIMIT $2 OFFSET $3"#
            )
            .bind(format!("%{}%", query))
            .bind(size)
            .bind(offset)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM inv_uom WHERE code ILIKE $1 OR name ILIKE $1"#
            )
            .bind(format!("%{}%", query))
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, crate::domain::models::Uom>(
                r#"SELECT * FROM inv_uom ORDER BY code LIMIT $1 OFFSET $2"#
            )
            .bind(size)
            .bind(offset)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM inv_uom"#
            )
            .fetch_one(self.db)
            .await?;
            (r, t)
        };

        Ok((rows, total))
    }

    pub async fn update(&self, id: uuid::Uuid, name: Option<&str>) -> anyhow::Result<Option<crate::domain::models::Uom>> {
        if let Some(name) = name {
            sqlx::query(
                r#"UPDATE inv_uom SET name=$1, updated_at=$2 WHERE id=$3"#
            )
            .bind(name)
            .bind(chrono::Utc::now())
            .bind(id)
            .execute(self.db)
            .await?;
        }

        let rec = sqlx::query_as::<_, crate::domain::models::Uom>(
            r#"SELECT * FROM inv_uom WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?;

        Ok(rec)
    }
}

// Supplier Repository
pub struct SupplierRepo<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> SupplierRepo<'a> {
    pub async fn create(&self, s: &crate::domain::models::Supplier) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO inv_suppliers(id,code,name,phone,email,address_line1,address_line2,city,province,country,postal_code,tax_id,status,created_at,updated_at,created_by,updated_by) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17)"#
        )
        .bind(&s.id)
        .bind(&s.code)
        .bind(&s.name)
        .bind(&s.phone)
        .bind(&s.email)
        .bind(&s.address_line1)
        .bind(&s.address_line2)
        .bind(&s.city)
        .bind(&s.province)
        .bind(&s.country)
        .bind(&s.postal_code)
        .bind(&s.tax_id)
        .bind(&s.status)
        .bind(&s.created_at)
        .bind(&s.updated_at)
        .bind(&s.created_by)
        .bind(&s.updated_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_paged(&self, q: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<crate::domain::models::Supplier>, i64)> {
        let offset = (page - 1) * size;
        let (rows, total) = if let Some(query) = q {
            let r = sqlx::query_as::<_, crate::domain::models::Supplier>(
                r#"SELECT * FROM inv_suppliers WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY code LIMIT $2 OFFSET $3"#
            )
            .bind(format!("%{}%", query))
            .bind(size)
            .bind(offset)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM inv_suppliers WHERE code ILIKE $1 OR name ILIKE $1"#
            )
            .bind(format!("%{}%", query))
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, crate::domain::models::Supplier>(
                r#"SELECT * FROM inv_suppliers ORDER BY code LIMIT $1 OFFSET $2"#
            )
            .bind(size)
            .bind(offset)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM inv_suppliers"#
            )
            .fetch_one(self.db)
            .await?;
            (r, t)
        };

        Ok((rows, total))
    }

    pub async fn update(&self, id: uuid::Uuid, name: Option<&str>, phone: Option<&str>, email: Option<&str>) -> anyhow::Result<Option<crate::domain::models::Supplier>> {
        if name.is_some() || phone.is_some() || email.is_some() {
            sqlx::query(
                r#"UPDATE inv_suppliers SET name=COALESCE($1,name), phone=COALESCE($2,phone), email=COALESCE($3,email), updated_at=$4 WHERE id=$5"#
            )
            .bind(name)
            .bind(phone)
            .bind(email)
            .bind(chrono::Utc::now())
            .bind(id)
            .execute(self.db)
            .await?;
        }

        let rec = sqlx::query_as::<_, crate::domain::models::Supplier>(
            r#"SELECT * FROM inv_suppliers WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?;

        Ok(rec)
    }
}
