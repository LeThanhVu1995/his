use anyhow::Result;
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::order::ClinicalOrder;

pub struct OrderRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> OrderRepo<'a> {
    // Clinical Order CRUD operations
    pub async fn create_order(&self, order: &ClinicalOrder) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO clinical_order (
                order_id, encounter_id, patient_id, order_type, status,
                ordered_by, ordered_at, priority_code, remarks
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(&order.order_id)
        .bind(&order.encounter_id)
        .bind(&order.patient_id)
        .bind(&order.order_type)
        .bind(&order.status)
        .bind(&order.ordered_by)
        .bind(&order.ordered_at)
        .bind(&order.priority_code)
        .bind(&order.remarks)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_order(&self, order_id: &str) -> Result<Option<ClinicalOrder>> {
        let order = sqlx::query_as::<_, ClinicalOrder>(
            r#"
            SELECT order_id, encounter_id, patient_id, order_type, status,
                   ordered_by, ordered_at, priority_code, remarks
            FROM clinical_order
            WHERE order_id = $1
            "#
        )
        .bind(order_id)
        .fetch_optional(self.db)
        .await?;
        Ok(order)
    }

    pub async fn list_patient_orders(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<ClinicalOrder>> {
        let orders = sqlx::query_as::<_, ClinicalOrder>(
            r#"
            SELECT order_id, encounter_id, patient_id, order_type, status,
                   ordered_by, ordered_at, priority_code, remarks
            FROM clinical_order
            WHERE patient_id = $1
            ORDER BY ordered_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(orders)
    }

    pub async fn list_encounter_orders(&self, encounter_id: &str, limit: i64, offset: i64) -> Result<Vec<ClinicalOrder>> {
        let orders = sqlx::query_as::<_, ClinicalOrder>(
            r#"
            SELECT order_id, encounter_id, patient_id, order_type, status,
                   ordered_by, ordered_at, priority_code, remarks
            FROM clinical_order
            WHERE encounter_id = $1
            ORDER BY ordered_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(encounter_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(orders)
    }

    pub async fn list_orders_by_type(&self, order_type: &str, limit: i64, offset: i64) -> Result<Vec<ClinicalOrder>> {
        let orders = sqlx::query_as::<_, ClinicalOrder>(
            r#"
            SELECT order_id, encounter_id, patient_id, order_type, status,
                   ordered_by, ordered_at, priority_code, remarks
            FROM clinical_order
            WHERE order_type = $1
            ORDER BY ordered_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(order_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(orders)
    }

    pub async fn list_orders_by_status(&self, status: &str, limit: i64, offset: i64) -> Result<Vec<ClinicalOrder>> {
        let orders = sqlx::query_as::<_, ClinicalOrder>(
            r#"
            SELECT order_id, encounter_id, patient_id, order_type, status,
                   ordered_by, ordered_at, priority_code, remarks
            FROM clinical_order
            WHERE status = $1
            ORDER BY ordered_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(orders)
    }

    pub async fn list_orders_by_priority(&self, priority: &str, limit: i64, offset: i64) -> Result<Vec<ClinicalOrder>> {
        let orders = sqlx::query_as::<_, ClinicalOrder>(
            r#"
            SELECT order_id, encounter_id, patient_id, order_type, status,
                   ordered_by, ordered_at, priority_code, remarks
            FROM clinical_order
            WHERE priority_code = $1
            ORDER BY ordered_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(priority)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(orders)
    }

    pub async fn list_orders_by_ordering_provider(&self, provider_id: &str, limit: i64, offset: i64) -> Result<Vec<ClinicalOrder>> {
        let orders = sqlx::query_as::<_, ClinicalOrder>(
            r#"
            SELECT order_id, encounter_id, patient_id, order_type, status,
                   ordered_by, ordered_at, priority_code, remarks
            FROM clinical_order
            WHERE ordered_by = $1
            ORDER BY ordered_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(provider_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(orders)
    }

    pub async fn update_order(&self, order: &ClinicalOrder) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE clinical_order SET
                encounter_id = $2, patient_id = $3, order_type = $4, status = $5,
                ordered_by = $6, ordered_at = $7, priority_code = $8, remarks = $9
            WHERE order_id = $1
            "#
        )
        .bind(&order.order_id)
        .bind(&order.encounter_id)
        .bind(&order.patient_id)
        .bind(&order.order_type)
        .bind(&order.status)
        .bind(&order.ordered_by)
        .bind(&order.ordered_at)
        .bind(&order.priority_code)
        .bind(&order.remarks)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn complete_order(&self, order_id: &str, user_id: &str) -> Result<ClinicalOrder> {
        sqlx::query(
            r#"
            UPDATE clinical_order SET
                status = 'completed'
            WHERE order_id = $1
            "#
        )
        .bind(order_id)
        .execute(self.db)
        .await?;

        // Return updated order
        self.get_order(order_id).await?.ok_or_else(|| anyhow::anyhow!("Order not found"))
    }

    pub async fn cancel_order(&self, order_id: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE clinical_order SET
                status = 'cancelled'
            WHERE order_id = $1
            "#
        )
        .bind(order_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn count_patient_orders(&self, patient_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM clinical_order
            WHERE patient_id = $1 AND status != 'deleted'
            "#
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_encounter_orders(&self, encounter_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM clinical_order
            WHERE encounter_id = $1 AND status != 'deleted'
            "#
        )
        .bind(encounter_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_orders_by_type(&self, order_type: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM clinical_order
            WHERE order_type = $1
            "#
        )
        .bind(order_type)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_orders_by_status(&self, status: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM clinical_order
            WHERE status = $1
            "#
        )
        .bind(status)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_orders_by_encounter_type(&self, encounter_id: &str, order_type: &str, status: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM clinical_order
            WHERE encounter_id = $1 AND order_type = $2 AND status = $3
            "#
        )
        .bind(encounter_id)
        .bind(order_type)
        .bind(status)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_orders_by_encounter_type_status(&self, encounter_id: &str, order_type: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM clinical_order
            WHERE encounter_id = $1 AND order_type = $2
            "#
        )
        .bind(encounter_id)
        .bind(order_type)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_orders_by_encounter_status(&self, encounter_id: &str, status: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM clinical_order
            WHERE encounter_id = $1 AND status = $2
            "#
        )
        .bind(encounter_id)
        .bind(status)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_orders_by_encounter(&self, encounter_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM clinical_order
            WHERE encounter_id = $1
            "#
        )
        .bind(encounter_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn delete_order(&self, order_id: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE clinical_order SET
                status = 'deleted'
            WHERE order_id = $1
            "#
        )
        .bind(order_id)
        .execute(self.db)
        .await?;
        Ok(())
    }
}
