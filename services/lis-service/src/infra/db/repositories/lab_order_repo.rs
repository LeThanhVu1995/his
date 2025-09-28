use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::entities::lab_order::{LabTestCatalog, LabOrder};
use anyhow::Result;

pub struct LabTestRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> LabTestRepo<'a> {
    pub async fn create(&self, test: &LabTestCatalog) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO lab_test_catalog (test_id, code, name, specimen_code, method_text, loinc_code)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(test.test_id)
        .bind(&test.code)
        .bind(&test.name)
        .bind(&test.specimen_code)
        .bind(&test.method_text)
        .bind(&test.loinc_code)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, test_id: Uuid) -> Result<Option<LabTestCatalog>> {
        Ok(sqlx::query_as::<_, LabTestCatalog>(
            r#"
            SELECT test_id, code, name, specimen_code, method_text, loinc_code
            FROM lab_test_catalog
            WHERE test_id = $1
            "#
        )
        .bind(test_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(
        &self,
        code: Option<String>,
        specimen_code: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LabTestCatalog>> {
        let mut query = r#"
            SELECT test_id, code, name, specimen_code, method_text, loinc_code
            FROM lab_test_catalog
            WHERE 1 = 1
        "#.to_string();

        if let Some(c) = code {
            query.push_str(&format!(" AND code ILIKE '%{}%'", c));
        }
        if let Some(s) = specimen_code {
            query.push_str(&format!(" AND specimen_code = '{}'", s));
        }

        query.push_str(&format!(" ORDER BY code LIMIT {} OFFSET {}", limit, offset));

        let tests = sqlx::query_as::<_, LabTestCatalog>(&query)
            .fetch_all(self.db)
            .await?;
        Ok(tests)
    }

    pub async fn update(&self, test_id: Uuid, test: &LabTestCatalog) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE lab_test_catalog
            SET name = $1, specimen_code = $2, method_text = $3, loinc_code = $4
            WHERE test_id = $5
            "#
        )
        .bind(&test.name)
        .bind(&test.specimen_code)
        .bind(&test.method_text)
        .bind(&test.loinc_code)
        .bind(test_id)
        .execute(self.db)
        .await?;
        Ok(())
    }
}

pub struct LabOrderRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> LabOrderRepo<'a> {
    pub async fn create(&self, order: &LabOrder) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO lab_order (lab_order_id, order_id, collected_at, collected_by, status)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(order.lab_order_id)
        .bind(order.order_id)
        .bind(order.collected_at)
        .bind(order.collected_by)
        .bind(&order.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, lab_order_id: Uuid) -> Result<Option<LabOrder>> {
        Ok(sqlx::query_as::<_, LabOrder>(
            r#"
            SELECT lab_order_id, order_id, collected_at, collected_by, status
            FROM lab_order
            WHERE lab_order_id = $1
            "#
        )
        .bind(lab_order_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(
        &self,
        order_id: Option<Uuid>,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LabOrder>> {
        let mut query = r#"
            SELECT lab_order_id, order_id, collected_at, collected_by, status
            FROM lab_order
            WHERE 1 = 1
        "#.to_string();

        if let Some(o_id) = order_id {
            query.push_str(&format!(" AND order_id = '{}'", o_id));
        }
        if let Some(s) = status {
            query.push_str(&format!(" AND status = '{}'", s));
        }

        query.push_str(&format!(" ORDER BY lab_order_id DESC LIMIT {} OFFSET {}", limit, offset));

        let orders = sqlx::query_as::<_, LabOrder>(&query)
            .fetch_all(self.db)
            .await?;
        Ok(orders)
    }
}
