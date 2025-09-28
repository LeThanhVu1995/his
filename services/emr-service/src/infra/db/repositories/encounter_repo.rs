use anyhow::Result;
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::problem::{Encounter, ClinicalNote};

pub struct EncounterRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> EncounterRepo<'a> {
    // Encounter CRUD operations
    pub async fn create_encounter(&self, encounter: &Encounter) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO encounter (
                encounter_id, patient_id, facility_id, type_code, status, start_time,
                end_time, attending_staff_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(&encounter.encounter_id)
        .bind(&encounter.patient_id)
        .bind(&encounter.facility_id)
        .bind(&encounter.type_code)
        .bind(&encounter.status)
        .bind(&encounter.start_time)
        .bind(&encounter.end_time)
        .bind(&encounter.attending_staff_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_encounter(&self, encounter_id: &str) -> Result<Option<Encounter>> {
        let encounter = sqlx::query_as::<_, Encounter>(
            r#"
            SELECT encounter_id, patient_id, facility_id, type_code, status, start_time,
                   end_time, attending_staff_id
            FROM encounter
            WHERE encounter_id = $1
            "#
        )
        .bind(encounter_id)
        .fetch_optional(self.db)
        .await?;
        Ok(encounter)
    }

    pub async fn list_patient_encounters(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<Encounter>> {
        let encounters = sqlx::query_as::<_, Encounter>(
            r#"
            SELECT encounter_id, patient_id, facility_id, type_code, status, start_time,
                   end_time, attending_staff_id
            FROM encounter
            WHERE patient_id = $1
            ORDER BY start_time DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(encounters)
    }

    pub async fn list_encounters_by_facility(&self, facility_id: &str, limit: i64, offset: i64) -> Result<Vec<Encounter>> {
        let encounters = sqlx::query_as::<_, Encounter>(
            r#"
            SELECT encounter_id, patient_id, facility_id, type_code, status, start_time,
                   end_time, attending_staff_id
            FROM encounter
            WHERE facility_id = $1
            ORDER BY start_time DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(facility_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(encounters)
    }

    pub async fn list_encounters_by_type(&self, encounter_type: &str, limit: i64, offset: i64) -> Result<Vec<Encounter>> {
        let encounters = sqlx::query_as::<_, Encounter>(
            r#"
            SELECT encounter_id, patient_id, facility_id, type_code, status, start_time,
                   end_time, attending_staff_id
            FROM encounter
            WHERE type_code = $1
            ORDER BY start_time DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(encounter_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(encounters)
    }

    pub async fn list_encounters_by_status(&self, status: &str, limit: i64, offset: i64) -> Result<Vec<Encounter>> {
        let encounters = sqlx::query_as::<_, Encounter>(
            r#"
            SELECT encounter_id, patient_id, facility_id, type_code, status, start_time,
                   end_time, attending_staff_id
            FROM encounter
            WHERE status = $1
            ORDER BY start_time DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(encounters)
    }

    pub async fn update_encounter(&self, encounter: &Encounter) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE encounter SET
                patient_id = $2, facility_id = $3, type_code = $4, status = $5,
                start_time = $6, end_time = $7, attending_staff_id = $8
            WHERE encounter_id = $1
            "#
        )
        .bind(&encounter.encounter_id)
        .bind(&encounter.patient_id)
        .bind(&encounter.facility_id)
        .bind(&encounter.type_code)
        .bind(&encounter.status)
        .bind(&encounter.start_time)
        .bind(&encounter.end_time)
        .bind(&encounter.attending_staff_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn close_encounter(&self, encounter_id: &str, end_time: chrono::DateTime<chrono::Utc>, _user_id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE encounter SET status = 'finished', end_time = $2 WHERE encounter_id = $1"
        )
        .bind(encounter_id)
        .bind(end_time)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn count_patient_encounters(&self, patient_id: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM encounter WHERE patient_id = $1"
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count.0)
    }

    pub async fn count_encounters_by_facility(&self, facility_id: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM encounter WHERE facility_id = $1"
        )
        .bind(facility_id)
        .fetch_one(self.db)
        .await?;
        Ok(count.0)
    }

    pub async fn count_encounters_by_type(&self, encounter_type: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM encounter WHERE type_code = $1"
        )
        .bind(encounter_type)
        .fetch_one(self.db)
        .await?;
        Ok(count.0)
    }

    pub async fn count_encounters_by_status(&self, status: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM encounter WHERE status = $1"
        )
        .bind(status)
        .fetch_one(self.db)
        .await?;
        Ok(count.0)
    }

    // Clinical Note operations
    pub async fn create_clinical_note(&self, note: &ClinicalNote) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO clinical_note (
                note_id, encounter_id, category_code, content_text, author_staff_id, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(&note.note_id)
        .bind(&note.encounter_id)
        .bind(&note.category_code)
        .bind(&note.content_text)
        .bind(&note.author_staff_id)
        .bind(&note.created_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_clinical_note(&self, note_id: &str) -> Result<Option<ClinicalNote>> {
        let note = sqlx::query_as::<_, ClinicalNote>(
            "SELECT note_id, encounter_id, category_code, content_text, author_staff_id, created_at FROM clinical_note WHERE note_id = $1"
        )
        .bind(note_id)
        .fetch_optional(self.db)
        .await?;
        Ok(note)
    }

    pub async fn list_encounter_notes(&self, encounter_id: &str, limit: i64, offset: i64) -> Result<Vec<ClinicalNote>> {
        let notes = sqlx::query_as::<_, ClinicalNote>(
            "SELECT note_id, encounter_id, category_code, content_text, author_staff_id, created_at FROM clinical_note WHERE encounter_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(encounter_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(notes)
    }

    pub async fn list_notes_by_type(&self, note_type: &str, limit: i64, offset: i64) -> Result<Vec<ClinicalNote>> {
        let notes = sqlx::query_as::<_, ClinicalNote>(
            "SELECT note_id, encounter_id, category_code, content_text, author_staff_id, created_at FROM clinical_note WHERE category_code = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(note_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(notes)
    }

    pub async fn update_clinical_note(&self, note: &ClinicalNote) -> Result<()> {
        sqlx::query(
            "UPDATE clinical_note SET encounter_id = $2, category_code = $3, content_text = $4, author_staff_id = $5, created_at = $6 WHERE note_id = $1"
        )
        .bind(&note.note_id)
        .bind(&note.encounter_id)
        .bind(&note.category_code)
        .bind(&note.content_text)
        .bind(&note.author_staff_id)
        .bind(&note.created_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_clinical_note(&self, note_id: &str, _user_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM clinical_note WHERE note_id = $1")
        .bind(note_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn count_encounter_notes(&self, encounter_id: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM clinical_note WHERE encounter_id = $1"
        )
        .bind(encounter_id)
        .fetch_one(self.db)
        .await?;
        Ok(count.0)
    }

    pub async fn count_notes_by_type(&self, note_type: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM clinical_note WHERE category_code = $1"
        )
        .bind(note_type)
        .fetch_one(self.db)
        .await?;
        Ok(count.0)
    }
}
