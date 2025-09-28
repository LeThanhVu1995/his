use sqlx::{Pool, Postgres};
use crate::domain::models::InvUom;
use anyhow::Result;

pub struct InvUomRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> InvUomRepo<'a> {
    pub async fn create(&self, uom: &InvUom) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO inv_uom (uom_id, code, name)
            VALUES ($1, $2, $3)
            "#
        )
        .bind(&uom.uom_id)
        .bind(&uom.code)
        .bind(&uom.name)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, uom_id: &str) -> Result<Option<InvUom>> {
        Ok(sqlx::query_as::<_, InvUom>(
            r#"
            SELECT uom_id, code, name
            FROM inv_uom
            WHERE uom_id = $1
            "#
        )
        .bind(uom_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn get_by_code(&self, code: &str) -> Result<Option<InvUom>> {
        Ok(sqlx::query_as::<_, InvUom>(
            r#"
            SELECT uom_id, code, name
            FROM inv_uom
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
        limit: i64,
        offset: i64,
    ) -> Result<Vec<InvUom>> {
        let mut query = r#"
            SELECT uom_id, code, name
            FROM inv_uom
            WHERE 1 = 1
        "#.to_string();

        if let Some(c) = code {
            query.push_str(&format!(" AND code ILIKE '%{}%'", c));
        }

        query.push_str(&format!(" ORDER BY code LIMIT {} OFFSET {}", limit, offset));

        let uoms = sqlx::query_as::<_, InvUom>(&query)
            .fetch_all(self.db)
            .await?;
        Ok(uoms)
    }

    pub async fn update(&self, uom_id: &str, uom: &InvUom) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE inv_uom
            SET name = $1
            WHERE uom_id = $2
            "#
        )
        .bind(&uom.name)
        .bind(uom_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, uom_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM inv_uom WHERE uom_id = $1")
            .bind(uom_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}
