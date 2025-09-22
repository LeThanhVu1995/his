use uuid::Uuid;
// use rust_decimal::Decimal;
use crate::domain::models::*;
use crate::domain::repo::{StockRepo, MovementRepo};

pub struct InventoryService<'a> {
    pub stocks: StockRepo<'a>,
    pub movements: MovementRepo<'a>,
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> InventoryService<'a> {
    pub async fn receive_stocks(
        &self,
        dst_wh: Uuid,
        lines: Vec<MoveLineReq>,
        note: Option<String>,
        by: Option<&str>,
    ) -> anyhow::Result<Uuid> {
        let mut tx = self.db.begin().await?;

        let mv_id = Uuid::new_v4();
        let mv_no = format!("REC-{}", &mv_id.to_string()[..8]);

        let movement = Movement {
            id: mv_id,
            mv_no,
            mv_type: "RECEIVE".into(),
            src_wh: None,
            dst_wh: Some(dst_wh),
            note,
            created_by: by.map(|s| s.to_string()),
            created_at: chrono::Utc::now(),
        };

        let movement_lines: Vec<MovementLine> = lines.into_iter().map(|line| MovementLine {
            id: Uuid::new_v4(),
            movement_id: mv_id,
            item_id: line.item_id,
            lot_id: line.lot_id,
            qty: line.qty,
        }).collect();

        self.movements.create(&movement, &movement_lines, &mut tx).await?;

        // Update stocks
        for line in &movement_lines {
            sqlx::query(
                r#"INSERT INTO inv_stocks(warehouse_id,item_id,lot_id,qty) VALUES($1,$2,$3,$4)
                   ON CONFLICT (warehouse_id,item_id,lot_id) DO UPDATE SET qty = inv_stocks.qty + EXCLUDED.qty"#
            )
            .bind(dst_wh)
            .bind(line.item_id)
            .bind(line.lot_id)
            .bind(line.qty)
            .execute(tx.as_mut())
            .await?;
        }

        tx.commit().await?;
        Ok(mv_id)
    }

    pub async fn issue_stocks(
        &self,
        src_wh: Uuid,
        lines: Vec<MoveLineReq>,
        note: Option<String>,
        by: Option<&str>,
    ) -> anyhow::Result<Uuid> {
        let mut tx = self.db.begin().await?;

        let mv_id = Uuid::new_v4();
        let mv_no = format!("ISS-{}", &mv_id.to_string()[..8]);

        let movement = Movement {
            id: mv_id,
            mv_no,
            mv_type: "ISSUE".into(),
            src_wh: Some(src_wh),
            dst_wh: None,
            note,
            created_by: by.map(|s| s.to_string()),
            created_at: chrono::Utc::now(),
        };

        let movement_lines: Vec<MovementLine> = lines.into_iter().map(|line| MovementLine {
            id: Uuid::new_v4(),
            movement_id: mv_id,
            item_id: line.item_id,
            lot_id: line.lot_id,
            qty: line.qty,
        }).collect();

        self.movements.create(&movement, &movement_lines, &mut tx).await?;

        // Update stocks
        for line in &movement_lines {
            sqlx::query(
                r#"UPDATE inv_stocks SET qty = qty - $4 WHERE warehouse_id=$1 AND item_id=$2 AND lot_id=$3"#
            )
            .bind(src_wh)
            .bind(line.item_id)
            .bind(line.lot_id)
            .bind(line.qty)
            .execute(tx.as_mut())
            .await?;
        }

        tx.commit().await?;
        Ok(mv_id)
    }

    pub async fn transfer_stocks(
        &self,
        src_wh: Uuid,
        dst_wh: Uuid,
        lines: Vec<MoveLineReq>,
        note: Option<String>,
        by: Option<&str>,
    ) -> anyhow::Result<Uuid> {
        let mut tx = self.db.begin().await?;

        let mv_id = Uuid::new_v4();
        let mv_no = format!("TRF-{}", &mv_id.to_string()[..8]);

        let movement = Movement {
            id: mv_id,
            mv_no,
            mv_type: "TRANSFER".into(),
            src_wh: Some(src_wh),
            dst_wh: Some(dst_wh),
            note,
            created_by: by.map(|s| s.to_string()),
            created_at: chrono::Utc::now(),
        };

        let movement_lines: Vec<MovementLine> = lines.into_iter().map(|line| MovementLine {
            id: Uuid::new_v4(),
            movement_id: mv_id,
            item_id: line.item_id,
            lot_id: line.lot_id,
            qty: line.qty,
        }).collect();

        self.movements.create(&movement, &movement_lines, &mut tx).await?;

        // Update stocks
        for line in &movement_lines {
            // Reduce from source
            sqlx::query(
                r#"UPDATE inv_stocks SET qty = qty - $4 WHERE warehouse_id=$1 AND item_id=$2 AND lot_id=$3"#
            )
            .bind(src_wh)
            .bind(line.item_id)
            .bind(line.lot_id)
            .bind(line.qty)
            .execute(tx.as_mut())
            .await?;

            // Add to destination
            sqlx::query(
                r#"INSERT INTO inv_stocks(warehouse_id,item_id,lot_id,qty) VALUES($1,$2,$3,$4)
                   ON CONFLICT (warehouse_id,item_id,lot_id) DO UPDATE SET qty = inv_stocks.qty + EXCLUDED.qty"#
            )
            .bind(dst_wh)
            .bind(line.item_id)
            .bind(line.lot_id)
            .bind(line.qty)
            .execute(tx.as_mut())
            .await?;
        }

        tx.commit().await?;
        Ok(mv_id)
    }

    pub async fn adjust_stocks(
        &self,
        wh: Uuid,
        lines: Vec<AdjustLineReq>,
        note: Option<String>,
        by: Option<&str>,
    ) -> anyhow::Result<Uuid> {
        let mut tx = self.db.begin().await?;

        let mv_id = Uuid::new_v4();
        let mv_no = format!("ADJ-{}", &mv_id.to_string()[..8]);

        let movement = Movement {
            id: mv_id,
            mv_no,
            mv_type: "ADJUST".into(),
            src_wh: None,
            dst_wh: Some(wh),
            note,
            created_by: by.map(|s| s.to_string()),
            created_at: chrono::Utc::now(),
        };

        let movement_lines: Vec<MovementLine> = lines.into_iter().map(|line| MovementLine {
            id: Uuid::new_v4(),
            movement_id: mv_id,
            item_id: line.item_id,
            lot_id: line.lot_id,
            qty: line.diff,
        }).collect();

        self.movements.create(&movement, &movement_lines, &mut tx).await?;

        // Update stocks
        for line in &movement_lines {
            sqlx::query(
                r#"INSERT INTO inv_stocks(warehouse_id,item_id,lot_id,qty) VALUES($1,$2,$3,$4)
                   ON CONFLICT (warehouse_id,item_id,lot_id) DO UPDATE SET qty = inv_stocks.qty + EXCLUDED.qty"#
            )
            .bind(wh)
            .bind(line.item_id)
            .bind(line.lot_id)
            .bind(line.qty)
            .execute(tx.as_mut())
            .await?;
        }

        tx.commit().await?;
        Ok(mv_id)
    }
}

// Use structs from dto module
use crate::dto::movement_dto::{MoveLineReq, AdjustLineReq};
