use anyhow::Result;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::drug_catalog::{
    DrugCatalog, CreateDrugCatalogRequest, UpdateDrugCatalogRequest, DrugCatalogStats
};

pub struct DrugCatalogRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> DrugCatalogRepo<'a> {
    pub async fn create(&self, drug: &CreateDrugCatalogRequest) -> Result<DrugCatalog> {
        let new_drug = sqlx::query_as::<_, DrugCatalog>(
            r#"
            INSERT INTO drug_catalog (drug_id, code, name, generic_name, form_code, strength_text, atc_code, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&drug.code)
        .bind(&drug.name)
        .bind(&drug.generic_name)
        .bind(&drug.form_code)
        .bind(&drug.strength_text)
        .bind(&drug.atc_code)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(self.db)
        .await?;

        Ok(new_drug)
    }

    pub async fn get_by_id(&self, drug_id: Uuid) -> Result<Option<DrugCatalog>> {
        let drug = sqlx::query_as::<_, DrugCatalog>(
            r#"
            SELECT * FROM drug_catalog WHERE drug_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(drug_id)
        .fetch_optional(self.db)
        .await?;

        Ok(drug)
    }

    pub async fn get_by_code(&self, code: &str) -> Result<Option<DrugCatalog>> {
        let drug = sqlx::query_as::<_, DrugCatalog>(
            r#"
            SELECT * FROM drug_catalog WHERE code = $1 AND deleted_at IS NULL
            "#
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?;

        Ok(drug)
    }

    pub async fn list_paged(
        &self,
        code: Option<String>,
        name: Option<String>,
        atc_code: Option<String>,
        form_code: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DrugCatalog>> {
        let mut query = r#"
            SELECT * FROM drug_catalog WHERE deleted_at IS NULL
        "#.to_string();
        let mut args = Vec::new();
        let mut arg_idx = 1;

        if let Some(c) = code {
            query.push_str(&format!(" AND code ILIKE ${}", arg_idx));
            args.push(format!("%{}%", c));
            arg_idx += 1;
        }
        if let Some(n) = name {
            query.push_str(&format!(" AND name ILIKE ${}", arg_idx));
            args.push(format!("%{}%", n));
            arg_idx += 1;
        }
        if let Some(atc) = atc_code {
            query.push_str(&format!(" AND atc_code = ${}", arg_idx));
            args.push(atc);
            arg_idx += 1;
        }
        if let Some(form) = form_code {
            query.push_str(&format!(" AND form_code = ${}", arg_idx));
            args.push(form);
            arg_idx += 1;
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", arg_idx, arg_idx + 1));

        let drugs = sqlx::query_as::<_, DrugCatalog>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.db)
            .await?;
        Ok(drugs)
    }

    pub async fn update(&self, drug_id: Uuid, drug: &UpdateDrugCatalogRequest) -> Result<DrugCatalog> {
        let updated_drug = sqlx::query_as::<_, DrugCatalog>(
            r#"
            UPDATE drug_catalog
            SET code = COALESCE($2, code),
                name = COALESCE($3, name),
                generic_name = COALESCE($4, generic_name),
                form_code = COALESCE($5, form_code),
                strength_text = COALESCE($6, strength_text),
                atc_code = COALESCE($7, atc_code),
                updated_at = $8
            WHERE drug_id = $1 AND deleted_at IS NULL
            RETURNING *
            "#
        )
        .bind(drug_id)
        .bind(&drug.code)
        .bind(&drug.name)
        .bind(&drug.generic_name)
        .bind(&drug.form_code)
        .bind(&drug.strength_text)
        .bind(&drug.atc_code)
        .bind(Utc::now())
        .fetch_one(self.db)
        .await?;

        Ok(updated_drug)
    }

    pub async fn delete(&self, drug_id: Uuid, deleted_by: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE drug_catalog
            SET deleted_at = $2, deleted_by = $3
            WHERE drug_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(drug_id)
        .bind(Utc::now())
        .bind(deleted_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_stats(&self) -> Result<DrugCatalogStats> {
        // For now, return placeholder stats
        // In a real implementation, you'd use raw query and map to struct
        Ok(DrugCatalogStats {
            total: 0,
            by_form: serde_json::Value::Object(serde_json::Map::new()),
            by_atc: serde_json::Value::Object(serde_json::Map::new()),
        })
    }
}
