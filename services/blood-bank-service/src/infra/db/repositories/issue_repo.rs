use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::issue::Issue;

pub struct IssueRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> IssueRepo<'a> {
    pub async fn insert(&self, i: &Issue) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bb_issue(issue_id, unit_id, encounter_id, issued_at, issued_by, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(i.issue_id)
        .bind(i.unit_id)
        .bind(i.encounter_id)
        .bind(i.issued_at)
        .bind(i.issued_by)
        .bind(i.created_at)
        .bind(i.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, issue_id: Uuid) -> anyhow::Result<Option<Issue>> {
        Ok(sqlx::query_as::<_, Issue>(
            "SELECT issue_id, unit_id, encounter_id, issued_at, issued_by, created_at, updated_at
             FROM bb_issue WHERE issue_id = $1"
        )
        .bind(issue_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_encounter(&self, encounter_id: Uuid) -> anyhow::Result<Vec<Issue>> {
        Ok(sqlx::query_as::<_, Issue>(
            "SELECT issue_id, unit_id, encounter_id, issued_at, issued_by, created_at, updated_at
             FROM bb_issue WHERE encounter_id = $1 ORDER BY issued_at DESC"
        )
        .bind(encounter_id)
        .fetch_all(self.db)
        .await?)
    }
}
