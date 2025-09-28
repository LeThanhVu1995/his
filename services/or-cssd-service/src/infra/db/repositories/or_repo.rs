use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::entities::or_schedule::{
    OrCase, OrChecklist, OrCaseWithChecklist, OrCaseStats
};
use anyhow::Result;

pub struct OrCaseRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> OrCaseRepo<'a> {
    pub async fn create(&self, or_case: &OrCase) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO or_case (
                or_case_id, encounter_id, patient_id, scheduled_room_id,
                scheduled_start, scheduled_end, actual_start, actual_end,
                status, procedure_text, surgeon_staff_id, anesthetist_staff_id,
                created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#
        )
        .bind(or_case.or_case_id)
        .bind(or_case.encounter_id)
        .bind(or_case.patient_id)
        .bind(or_case.scheduled_room_id)
        .bind(or_case.scheduled_start)
        .bind(or_case.scheduled_end)
        .bind(or_case.actual_start)
        .bind(or_case.actual_end)
        .bind(&or_case.status)
        .bind(&or_case.procedure_text)
        .bind(or_case.surgeon_staff_id)
        .bind(or_case.anesthetist_staff_id)
        .bind(or_case.created_by)
        .bind(or_case.updated_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, or_case_id: Uuid) -> Result<Option<OrCase>> {
        Ok(sqlx::query_as::<_, OrCase>(
            r#"
            SELECT or_case_id, encounter_id, patient_id, scheduled_room_id,
                   scheduled_start, scheduled_end, actual_start, actual_end,
                   status, procedure_text, surgeon_staff_id, anesthetist_staff_id,
                   created_at, created_by, updated_at, updated_by
            FROM or_case
            WHERE or_case_id = $1
            "#
        )
        .bind(or_case_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(
        &self,
        status: Option<String>,
        patient_id: Option<Uuid>,
        surgeon_staff_id: Option<Uuid>,
        scheduled_room_id: Option<Uuid>,
        date_from: Option<chrono::DateTime<chrono::Utc>>,
        date_to: Option<chrono::DateTime<chrono::Utc>>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<OrCase>> {
        let mut query = r#"
            SELECT or_case_id, encounter_id, patient_id, scheduled_room_id,
                   scheduled_start, scheduled_end, actual_start, actual_end,
                   status, procedure_text, surgeon_staff_id, anesthetist_staff_id,
                   created_at, created_by, updated_at, updated_by
            FROM or_case
            WHERE 1 = 1
        "#.to_string();

        let mut args = Vec::new();
        let mut arg_idx = 1;

        if let Some(s) = status {
            query.push_str(&format!(" AND status = ${}", arg_idx));
            args.push(s);
            arg_idx += 1;
        }
        if let Some(p_id) = patient_id {
            query.push_str(&format!(" AND patient_id = ${}", arg_idx));
            args.push(p_id.to_string());
            arg_idx += 1;
        }
        if let Some(s_id) = surgeon_staff_id {
            query.push_str(&format!(" AND surgeon_staff_id = ${}", arg_idx));
            args.push(s_id.to_string());
            arg_idx += 1;
        }
        if let Some(r_id) = scheduled_room_id {
            query.push_str(&format!(" AND scheduled_room_id = ${}", arg_idx));
            args.push(r_id.to_string());
            arg_idx += 1;
        }
        if let Some(df) = date_from {
            query.push_str(&format!(" AND scheduled_start >= ${}", arg_idx));
            args.push(df.to_rfc3339());
            arg_idx += 1;
        }
        if let Some(dt) = date_to {
            query.push_str(&format!(" AND scheduled_start <= ${}", arg_idx));
            args.push(dt.to_rfc3339());
            arg_idx += 1;
        }

        query.push_str(&format!(" ORDER BY scheduled_start DESC LIMIT ${} OFFSET ${}", arg_idx, arg_idx + 1));

        let or_cases = sqlx::query_as::<_, OrCase>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.db)
            .await?;
        Ok(or_cases)
    }

    pub async fn update(&self, or_case_id: Uuid, or_case: &OrCase) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE or_case
            SET scheduled_room_id = $1, scheduled_start = $2, scheduled_end = $3,
                actual_start = $4, actual_end = $5, status = $6, procedure_text = $7,
                surgeon_staff_id = $8, anesthetist_staff_id = $9, updated_at = $10, updated_by = $11
            WHERE or_case_id = $12
            "#
        )
        .bind(&or_case.scheduled_room_id)
        .bind(&or_case.scheduled_start)
        .bind(&or_case.scheduled_end)
        .bind(&or_case.actual_start)
        .bind(&or_case.actual_end)
        .bind(&or_case.status)
        .bind(&or_case.procedure_text)
        .bind(&or_case.surgeon_staff_id)
        .bind(&or_case.anesthetist_staff_id)
        .bind(chrono::Utc::now())
        .bind(or_case.updated_by)
        .bind(or_case_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, or_case_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM or_case WHERE or_case_id = $1")
            .bind(or_case_id)
            .execute(self.db)
            .await?;
        Ok(())
    }

    pub async fn get_stats(&self) -> Result<OrCaseStats> {
        // For now, return placeholder stats
        // In a real implementation, you'd use raw query and map to struct
        Ok(OrCaseStats {
            total: 0,
            scheduled: 0,
            in_progress: 0,
            completed: 0,
            cancelled: 0,
        })
    }
}

pub struct OrChecklistRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> OrChecklistRepo<'a> {
    pub async fn create(&self, checklist: &OrChecklist) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO or_checklist (checklist_id, or_case_id, phase_code, item_code, completed, completed_at, completed_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(checklist.checklist_id)
        .bind(checklist.or_case_id)
        .bind(&checklist.phase_code)
        .bind(&checklist.item_code)
        .bind(&checklist.completed)
        .bind(checklist.completed_at)
        .bind(checklist.completed_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_or_case_id(&self, or_case_id: Uuid) -> Result<Vec<OrChecklist>> {
        let checklists = sqlx::query_as::<_, OrChecklist>(
            r#"
            SELECT checklist_id, or_case_id, phase_code, item_code, completed, completed_at, completed_by, created_at
            FROM or_checklist
            WHERE or_case_id = $1
            ORDER BY phase_code, item_code
            "#
        )
        .bind(or_case_id)
        .fetch_all(self.db)
        .await?;
        Ok(checklists)
    }

    pub async fn update(&self, checklist_id: Uuid, checklist: &OrChecklist) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE or_checklist
            SET completed = $1, completed_at = $2, completed_by = $3
            WHERE checklist_id = $4
            "#
        )
        .bind(&checklist.completed)
        .bind(checklist.completed_at)
        .bind(checklist.completed_by)
        .bind(checklist_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, checklist_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM or_checklist WHERE checklist_id = $1")
            .bind(checklist_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}
