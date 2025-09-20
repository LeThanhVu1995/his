use crate::domain::models::MasterCode;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct MasterRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> MasterRepo<'a> {

    pub async fn list_codes_paged(&self, category: Option<&str>, page: i64, page_size: i64) -> anyhow::Result<(Vec<MasterCode>, i64)> {
        let page = page.max(1);
        let page_size = page_size.clamp(1, 200);
        let offset = (page - 1) * page_size;

        let (items, total) = if let Some(cat) = category {
            let items = sqlx::query_as::<_, MasterCode>(r#"
                SELECT id, category, code, name, description, is_active, created_at, updated_at
                FROM master_codes WHERE category = $1 ORDER BY code OFFSET $2 LIMIT $3
            "#)
            .bind(cat)
            .bind(offset)
            .bind(page_size)
            .fetch_all(self.db).await?;
            let total: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM master_codes WHERE category = $1")
                .bind(cat)
                .fetch_one(self.db).await?;
            (items, total)
        } else {
            let items = sqlx::query_as::<_, MasterCode>(r#"
                SELECT id, category, code, name, description, is_active, created_at, updated_at
                FROM master_codes ORDER BY category, code OFFSET $1 LIMIT $2
            "#)
            .bind(offset)
            .bind(page_size)
            .fetch_all(self.db).await?;
            let total: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM master_codes").fetch_one(self.db).await?;
            (items, total)
        };
        Ok((items, total))
    }

    pub async fn search_codes_paged(
        &self,
        category: Option<&str>,
        search: Option<&str>,
        is_active: Option<bool>,
        page: i64,
        page_size: i64
    ) -> anyhow::Result<(Vec<MasterCode>, i64)> {
        let page = page.max(1);
        let page_size = page_size.clamp(1, 200);
        let offset = (page - 1) * page_size;

        // Use simple approach with separate queries for each case
        let (items, total) = match (category, search, is_active) {
            (Some(cat), Some(search_term), Some(active)) => {
                let items = sqlx::query_as::<_, MasterCode>(r#"
                    SELECT id, category, code, name, description, is_active, created_at, updated_at
                    FROM master_codes
                    WHERE category = $1 AND (name ILIKE $2 OR description ILIKE $2) AND is_active = $3
                    ORDER BY category, code OFFSET $4 LIMIT $5
                "#)
                .bind(cat)
                .bind(format!("%{}%", search_term))
                .bind(active)
                .bind(offset)
                .bind(page_size)
                .fetch_all(self.db).await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM master_codes
                    WHERE category = $1 AND (name ILIKE $2 OR description ILIKE $2) AND is_active = $3
                "#)
                .bind(cat)
                .bind(format!("%{}%", search_term))
                .bind(active)
                .fetch_one(self.db).await?;
                (items, total)
            }
            (Some(cat), Some(search_term), None) => {
                let items = sqlx::query_as::<_, MasterCode>(r#"
                    SELECT id, category, code, name, description, is_active, created_at, updated_at
                    FROM master_codes
                    WHERE category = $1 AND (name ILIKE $2 OR description ILIKE $2)
                    ORDER BY category, code OFFSET $3 LIMIT $4
                "#)
                .bind(cat)
                .bind(format!("%{}%", search_term))
                .bind(offset)
                .bind(page_size)
                .fetch_all(self.db).await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM master_codes
                    WHERE category = $1 AND (name ILIKE $2 OR description ILIKE $2)
                "#)
                .bind(cat)
                .bind(format!("%{}%", search_term))
                .fetch_one(self.db).await?;
                (items, total)
            }
            (Some(cat), None, Some(active)) => {
                let items = sqlx::query_as::<_, MasterCode>(r#"
                    SELECT id, category, code, name, description, is_active, created_at, updated_at
                    FROM master_codes
                    WHERE category = $1 AND is_active = $2
                    ORDER BY category, code OFFSET $3 LIMIT $4
                "#)
                .bind(cat)
                .bind(active)
                .bind(offset)
                .bind(page_size)
                .fetch_all(self.db).await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM master_codes
                    WHERE category = $1 AND is_active = $2
                "#)
                .bind(cat)
                .bind(active)
                .fetch_one(self.db).await?;
                (items, total)
            }
            (None, Some(search_term), Some(active)) => {
                let items = sqlx::query_as::<_, MasterCode>(r#"
                    SELECT id, category, code, name, description, is_active, created_at, updated_at
                    FROM master_codes
                    WHERE (name ILIKE $1 OR description ILIKE $1) AND is_active = $2
                    ORDER BY category, code OFFSET $3 LIMIT $4
                "#)
                .bind(format!("%{}%", search_term))
                .bind(active)
                .bind(offset)
                .bind(page_size)
                .fetch_all(self.db).await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM master_codes
                    WHERE (name ILIKE $1 OR description ILIKE $1) AND is_active = $2
                "#)
                .bind(format!("%{}%", search_term))
                .bind(active)
                .fetch_one(self.db).await?;
                (items, total)
            }
            (Some(cat), None, None) => {
                // Use existing list_codes_paged for category only
                return self.list_codes_paged(Some(cat), page, page_size).await;
            }
            (None, Some(search_term), None) => {
                let items = sqlx::query_as::<_, MasterCode>(r#"
                    SELECT id, category, code, name, description, is_active, created_at, updated_at
                    FROM master_codes
                    WHERE name ILIKE $1 OR description ILIKE $1
                    ORDER BY category, code OFFSET $2 LIMIT $3
                "#)
                .bind(format!("%{}%", search_term))
                .bind(offset)
                .bind(page_size)
                .fetch_all(self.db).await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM master_codes
                    WHERE name ILIKE $1 OR description ILIKE $1
                "#)
                .bind(format!("%{}%", search_term))
                .fetch_one(self.db).await?;
                (items, total)
            }
            (None, None, Some(active)) => {
                let items = sqlx::query_as::<_, MasterCode>(r#"
                    SELECT id, category, code, name, description, is_active, created_at, updated_at
                    FROM master_codes
                    WHERE is_active = $1
                    ORDER BY category, code OFFSET $2 LIMIT $3
                "#)
                .bind(active)
                .bind(offset)
                .bind(page_size)
                .fetch_all(self.db).await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM master_codes
                    WHERE is_active = $1
                "#)
                .bind(active)
                .fetch_one(self.db).await?;
                (items, total)
            }
            (None, None, None) => {
                // Use existing list_codes_paged for no filters
                return self.list_codes_paged(None, page, page_size).await;
            }
        };

        Ok((items, total))
    }

    pub async fn find_code(&self, id: Uuid) -> anyhow::Result<Option<MasterCode>> {
        let rec = sqlx::query_as::<_, MasterCode>(r#"
            SELECT id, category, code, name, description, is_active, created_at, updated_at
            FROM master_codes WHERE id = $1
        "#)
        .bind(id)
        .fetch_optional(self.db).await?;
        Ok(rec)
    }

    pub async fn create_code(&self, category: &str, code: &str, name: &str, description: Option<&str>) -> anyhow::Result<MasterCode> {
        let id = Uuid::new_v4();
        let rec = sqlx::query_as::<_, MasterCode>(r#"
            INSERT INTO master_codes (id, category, code, name, description)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, category, code, name, description, is_active, created_at, updated_at
        "#)
        .bind(id)
        .bind(category)
        .bind(code)
        .bind(name)
        .bind(description)
        .fetch_one(self.db).await?;
        Ok(rec)
    }

    pub async fn update_code(&self, id: Uuid, name: Option<&str>, description: Option<&str>, is_active: Option<bool>) -> anyhow::Result<Option<MasterCode>> {
        let rec = sqlx::query_as::<_, MasterCode>(r#"
            UPDATE master_codes
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                is_active = COALESCE($4, is_active),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, category, code, name, description, is_active, created_at, updated_at
        "#)
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(is_active)
        .fetch_optional(self.db).await?;
        Ok(rec)
    }

    pub async fn delete_code(&self, id: Uuid) -> anyhow::Result<u64> {
        let res = sqlx::query("DELETE FROM master_codes WHERE id = $1")
            .bind(id)
            .execute(self.db).await?;
        Ok(res.rows_affected())
    }
}
