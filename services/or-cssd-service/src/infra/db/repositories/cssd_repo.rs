use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use crate::domain::entities::cssd_tray::{
    CssdTray, CssdTrayItem, CssdSterilizationLot, CssdLotItem,
    CssdTrayWithItems, CssdSterilizationLotWithItems, CssdStats
};
use anyhow::Result;

pub struct CssdTrayRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> CssdTrayRepo<'a> {
    pub async fn create(&self, tray: &CssdTray) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO cssd_tray (tray_id, code, name, description, created_by, updated_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(tray.tray_id)
        .bind(&tray.code)
        .bind(&tray.name)
        .bind(&tray.description)
        .bind(tray.created_by)
        .bind(tray.updated_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, tray_id: Uuid) -> Result<Option<CssdTray>> {
        Ok(sqlx::query_as::<_, CssdTray>(
            r#"
            SELECT tray_id, code, name, description, created_at, created_by, updated_at, updated_by
            FROM cssd_tray
            WHERE tray_id = $1
            "#
        )
        .bind(tray_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn get_by_code(&self, code: &str) -> Result<Option<CssdTray>> {
        Ok(sqlx::query_as::<_, CssdTray>(
            r#"
            SELECT tray_id, code, name, description, created_at, created_by, updated_at, updated_by
            FROM cssd_tray
            WHERE code = $1
            "#
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(
        &self,
        code: Option<String>,
        name: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CssdTray>> {
        let mut query = r#"
            SELECT tray_id, code, name, description, created_at, created_by, updated_at, updated_by
            FROM cssd_tray
            WHERE 1 = 1
        "#.to_string();

        if let Some(c) = code {
            query.push_str(&format!(" AND code ILIKE '%{}%'", c));
        }
        if let Some(n) = name {
            query.push_str(&format!(" AND name ILIKE '%{}%'", n));
        }

        query.push_str(&format!(" ORDER BY code LIMIT {} OFFSET {}", limit, offset));

        let trays = sqlx::query_as::<_, CssdTray>(&query)
            .fetch_all(self.db)
            .await?;
        Ok(trays)
    }

    pub async fn update(&self, tray_id: Uuid, tray: &CssdTray) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE cssd_tray
            SET name = $1, description = $2, updated_at = $3, updated_by = $4
            WHERE tray_id = $5
            "#
        )
        .bind(&tray.name)
        .bind(&tray.description)
        .bind(Utc::now())
        .bind(tray.updated_by)
        .bind(tray_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, tray_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM cssd_tray WHERE tray_id = $1")
            .bind(tray_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}

pub struct CssdTrayItemRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> CssdTrayItemRepo<'a> {
    pub async fn create(&self, item: &CssdTrayItem) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO cssd_tray_item (tray_item_id, tray_id, instrument_code, quantity)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(item.tray_item_id)
        .bind(item.tray_id)
        .bind(&item.instrument_code)
        .bind(item.quantity)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_tray_id(&self, tray_id: Uuid) -> Result<Vec<CssdTrayItem>> {
        let items = sqlx::query_as::<_, CssdTrayItem>(
            r#"
            SELECT tray_item_id, tray_id, instrument_code, quantity, created_at
            FROM cssd_tray_item
            WHERE tray_id = $1
            ORDER BY instrument_code
            "#
        )
        .bind(tray_id)
        .fetch_all(self.db)
        .await?;
        Ok(items)
    }

    pub async fn update(&self, tray_item_id: Uuid, item: &CssdTrayItem) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE cssd_tray_item
            SET instrument_code = $1, quantity = $2
            WHERE tray_item_id = $3
            "#
        )
        .bind(&item.instrument_code)
        .bind(item.quantity)
        .bind(tray_item_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, tray_item_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM cssd_tray_item WHERE tray_item_id = $1")
            .bind(tray_item_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}

pub struct CssdSterilizationLotRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> CssdSterilizationLotRepo<'a> {
    pub async fn create(&self, lot: &CssdSterilizationLot) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO cssd_sterilization_lot (lot_id, lot_code, method_code, started_at, completed_at, released_by, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(lot.lot_id)
        .bind(&lot.lot_code)
        .bind(&lot.method_code)
        .bind(lot.started_at)
        .bind(lot.completed_at)
        .bind(lot.released_by)
        .bind(&lot.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, lot_id: Uuid) -> Result<Option<CssdSterilizationLot>> {
        Ok(sqlx::query_as::<_, CssdSterilizationLot>(
            r#"
            SELECT lot_id, lot_code, method_code, started_at, completed_at, released_by, status, created_at
            FROM cssd_sterilization_lot
            WHERE lot_id = $1
            "#
        )
        .bind(lot_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(
        &self,
        lot_code: Option<String>,
        method_code: Option<String>,
        status: Option<String>,
        date_from: Option<DateTime<Utc>>,
        date_to: Option<DateTime<Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<CssdSterilizationLot>> {
        let mut query = r#"
            SELECT lot_id, lot_code, method_code, started_at, completed_at, released_by, status, created_at
            FROM cssd_sterilization_lot
            WHERE 1 = 1
        "#.to_string();

        if let Some(lc) = lot_code {
            query.push_str(&format!(" AND lot_code ILIKE '%{}%'", lc));
        }
        if let Some(mc) = method_code {
            query.push_str(&format!(" AND method_code = '{}'", mc));
        }
        if let Some(s) = status {
            query.push_str(&format!(" AND status = '{}'", s));
        }
        if let Some(df) = date_from {
            query.push_str(&format!(" AND started_at >= '{}'", df.format("%Y-%m-%d %H:%M:%S")));
        }
        if let Some(dt) = date_to {
            query.push_str(&format!(" AND started_at <= '{}'", dt.format("%Y-%m-%d %H:%M:%S")));
        }

        query.push_str(&format!(" ORDER BY started_at DESC LIMIT {} OFFSET {}", limit, offset));

        let lots = sqlx::query_as::<_, CssdSterilizationLot>(&query)
            .fetch_all(self.db)
            .await?;
        Ok(lots)
    }

    pub async fn update(&self, lot_id: Uuid, lot: &CssdSterilizationLot) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE cssd_sterilization_lot
            SET completed_at = $1, released_by = $2, status = $3
            WHERE lot_id = $4
            "#
        )
        .bind(&lot.completed_at)
        .bind(&lot.released_by)
        .bind(&lot.status)
        .bind(lot_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, lot_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM cssd_sterilization_lot WHERE lot_id = $1")
            .bind(lot_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}

pub struct CssdLotItemRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> CssdLotItemRepo<'a> {
    pub async fn create(&self, item: &CssdLotItem) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO cssd_lot_item (lot_item_id, lot_id, tray_id, expiry_date)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(item.lot_item_id)
        .bind(item.lot_id)
        .bind(item.tray_id)
        .bind(item.expiry_date)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_lot_id(&self, lot_id: Uuid) -> Result<Vec<CssdLotItem>> {
        let items = sqlx::query_as::<_, CssdLotItem>(
            r#"
            SELECT lot_item_id, lot_id, tray_id, expiry_date, created_at
            FROM cssd_lot_item
            WHERE lot_id = $1
            ORDER BY expiry_date
            "#
        )
        .bind(lot_id)
        .fetch_all(self.db)
        .await?;
        Ok(items)
    }

    pub async fn get_expired_items(&self) -> Result<Vec<CssdLotItem>> {
        let items = sqlx::query_as::<_, CssdLotItem>(
            r#"
            SELECT lot_item_id, lot_id, tray_id, expiry_date, created_at
            FROM cssd_lot_item
            WHERE expiry_date < CURRENT_DATE
            ORDER BY expiry_date
            "#
        )
        .fetch_all(self.db)
        .await?;
        Ok(items)
    }

    pub async fn delete(&self, lot_item_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM cssd_lot_item WHERE lot_item_id = $1")
            .bind(lot_item_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}
