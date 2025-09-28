use anyhow::Result;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::dispense::{
    Dispense, DispenseItem, CreateDispenseRequest, UpdateDispenseRequest,
    CreateDispenseItemRequest, UpdateDispenseItemRequest, DispenseWithItems, DispenseStats, ExpiredDispenseItem
};

pub struct DispenseRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> DispenseRepo<'a> {
    pub async fn create(&self, dispense: &CreateDispenseRequest) -> Result<Dispense> {
        let new_dispense = sqlx::query_as::<_, Dispense>(
            r#"
            INSERT INTO dispense (dispense_id, prescription_id, dispensed_by, dispensed_at, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'IN_PROGRESS', $5, $6)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(dispense.prescription_id)
        .bind(dispense.dispensed_by)
        .bind(dispense.dispensed_by.map(|_| Utc::now()))
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(self.db)
        .await?;

        Ok(new_dispense)
    }

    pub async fn create_item(&self, dispense_id: Uuid, item: &CreateDispenseItemRequest) -> Result<DispenseItem> {
        let new_item = sqlx::query_as::<_, DispenseItem>(
            r#"
            INSERT INTO dispense_item (
                dispense_item_id, dispense_id, prescription_item_id, quantity, unit,
                batch_id, expiry_date, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(dispense_id)
        .bind(item.prescription_item_id)
        .bind(item.quantity)
        .bind(&item.unit)
        .bind(item.batch_id)
        .bind(item.expiry_date)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(self.db)
        .await?;

        Ok(new_item)
    }

    pub async fn get_by_id(&self, dispense_id: Uuid) -> Result<Option<Dispense>> {
        let dispense = sqlx::query_as::<_, Dispense>(
            r#"
            SELECT * FROM dispense WHERE dispense_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(dispense_id)
        .fetch_optional(self.db)
        .await?;

        Ok(dispense)
    }

    pub async fn get_with_items(&self, dispense_id: Uuid) -> Result<Option<DispenseWithItems>> {
        let dispense = self.get_by_id(dispense_id).await?;
        if let Some(dispense) = dispense {
            let items = self.get_items_by_dispense_id(dispense_id).await?;
            Ok(Some(DispenseWithItems {
                dispense,
                items,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_items_by_dispense_id(&self, dispense_id: Uuid) -> Result<Vec<DispenseItem>> {
        let items = sqlx::query_as::<_, DispenseItem>(
            r#"
            SELECT * FROM dispense_item WHERE dispense_id = $1 AND deleted_at IS NULL
            ORDER BY created_at ASC
            "#
        )
        .bind(dispense_id)
        .fetch_all(self.db)
        .await?;
        Ok(items)
    }

    pub async fn list_paged(
        &self,
        prescription_id: Option<Uuid>,
        dispensed_by: Option<Uuid>,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Dispense>> {
        let mut query = r#"
            SELECT * FROM dispense WHERE deleted_at IS NULL
        "#.to_string();
        let mut args = Vec::new();
        let mut arg_idx = 1;

        if let Some(p_id) = prescription_id {
            query.push_str(&format!(" AND prescription_id = ${}", arg_idx));
            args.push(sqlx::query_scalar("SELECT $1::uuid").bind(p_id).fetch_one(self.db).await?);
            arg_idx += 1;
        }
        if let Some(d_id) = dispensed_by {
            query.push_str(&format!(" AND dispensed_by = ${}", arg_idx));
            args.push(sqlx::query_scalar("SELECT $1::uuid").bind(d_id).fetch_one(self.db).await?);
            arg_idx += 1;
        }
        if let Some(s) = status {
            query.push_str(&format!(" AND status = ${}", arg_idx));
            args.push(sqlx::query_scalar("SELECT $1::text").bind(s).fetch_one(self.db).await?);
            arg_idx += 1;
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", arg_idx, arg_idx + 1));

        let dispenses = sqlx::query_as::<_, Dispense>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.db)
            .await?;
        Ok(dispenses)
    }

    pub async fn update(&self, dispense_id: Uuid, dispense: &UpdateDispenseRequest) -> Result<Dispense> {
        let updated_dispense = sqlx::query_as::<_, Dispense>(
            r#"
            UPDATE dispense
            SET status = COALESCE($2, status),
                dispensed_by = COALESCE($3, dispensed_by),
                dispensed_at = COALESCE($4, dispensed_at),
                updated_at = $5
            WHERE dispense_id = $1 AND deleted_at IS NULL
            RETURNING *
            "#
        )
        .bind(dispense_id)
        .bind(&dispense.status)
        .bind(dispense.dispensed_by)
        .bind(dispense.dispensed_at)
        .bind(Utc::now())
        .fetch_one(self.db)
        .await?;

        Ok(updated_dispense)
    }

    pub async fn update_item(&self, dispense_item_id: Uuid, item: &UpdateDispenseItemRequest) -> Result<DispenseItem> {
        let updated_item = sqlx::query_as::<_, DispenseItem>(
            r#"
            UPDATE dispense_item
            SET quantity = COALESCE($2, quantity),
                unit = COALESCE($3, unit),
                batch_id = COALESCE($4, batch_id),
                expiry_date = COALESCE($5, expiry_date),
                updated_at = $6
            WHERE dispense_item_id = $1 AND deleted_at IS NULL
            RETURNING *
            "#
        )
        .bind(dispense_item_id)
        .bind(item.quantity)
        .bind(&item.unit)
        .bind(item.batch_id)
        .bind(item.expiry_date)
        .bind(Utc::now())
        .fetch_one(self.db)
        .await?;

        Ok(updated_item)
    }

    pub async fn delete(&self, dispense_id: Uuid, deleted_by: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE dispense
            SET deleted_at = $2, deleted_by = $3
            WHERE dispense_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(dispense_id)
        .bind(Utc::now())
        .bind(deleted_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_item(&self, dispense_item_id: Uuid, deleted_by: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE dispense_item
            SET deleted_at = $2, deleted_by = $3
            WHERE dispense_item_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(dispense_item_id)
        .bind(Utc::now())
        .bind(deleted_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_expired_items(&self, limit: i64, offset: i64) -> Result<Vec<ExpiredDispenseItem>> {
        let items = sqlx::query_as::<_, ExpiredDispenseItem>(
            r#"
            SELECT
                di.dispense_item_id,
                di.dispense_id,
                di.prescription_item_id,
                dc.name as drug_name,
                di.batch_id,
                di.expiry_date,
                di.quantity,
                di.unit
            FROM dispense_item di
            JOIN prescription_item pi ON di.prescription_item_id = pi.prescription_item_id
            JOIN drug_catalog dc ON pi.drug_id = dc.drug_id
            WHERE di.expiry_date < CURRENT_DATE AND di.deleted_at IS NULL
            ORDER BY di.expiry_date ASC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(items)
    }

    pub async fn get_stats(&self) -> Result<DispenseStats> {
        // For now, return placeholder stats
        // In a real implementation, you'd use raw query and map to struct
        Ok(DispenseStats {
            total: 0,
            in_progress: 0,
            completed: 0,
            cancelled: 0,
        })
    }

    // Placeholder methods for compatibility with existing handlers
    pub async fn find(&self, dispense_id: Uuid) -> Result<Option<Dispense>> {
        self.get_by_id(dispense_id).await
    }

    pub async fn finish(&self, dispense_id: Uuid) -> Result<Dispense> {
        let update_req = UpdateDispenseRequest {
            status: Some("COMPLETED".to_string()),
            dispensed_by: None,
            dispensed_at: Some(chrono::Utc::now()),
        };
        self.update(dispense_id, &update_req).await
    }
}