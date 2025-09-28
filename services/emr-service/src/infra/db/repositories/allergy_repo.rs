use anyhow::Result;
use chrono::NaiveDate;
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::allergy::{AllergyIntolerance, MedicationStatement};

pub struct AllergyRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> AllergyRepo<'a> {
    // Allergy Intolerance CRUD operations
    pub async fn create_allergy(&self, allergy: &AllergyIntolerance) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO allergy_intolerance (
                allergy_id, patient_id, substance_code, reaction_text,
                severity_code, status, recorded_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(&allergy.allergy_id)
        .bind(&allergy.patient_id)
        .bind(&allergy.substance_code)
        .bind(&allergy.reaction_text)
        .bind(&allergy.severity_code)
        .bind(&allergy.status)
        .bind(&allergy.recorded_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_allergy(&self, allergy_id: &str) -> Result<Option<AllergyIntolerance>> {
        let allergy = sqlx::query_as::<_, AllergyIntolerance>(
            r#"
            SELECT allergy_id, patient_id, substance_code, reaction_text,
                   severity_code, status, recorded_at
            FROM allergy_intolerance
            WHERE allergy_id = $1
            "#
        )
        .bind(allergy_id)
        .fetch_optional(self.db)
        .await?;
        Ok(allergy)
    }

    pub async fn list_patient_allergies(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<AllergyIntolerance>> {
        let allergies = sqlx::query_as::<_, AllergyIntolerance>(
            r#"
            SELECT allergy_id, patient_id, substance_code, reaction_text,
                   severity_code, status, recorded_at
            FROM allergy_intolerance
            WHERE patient_id = $1
            ORDER BY recorded_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(allergies)
    }

    pub async fn list_allergies_by_substance(&self, substance_code: &str, limit: i64, offset: i64) -> Result<Vec<AllergyIntolerance>> {
        let allergies = sqlx::query_as::<_, AllergyIntolerance>(
            r#"
            SELECT allergy_id, patient_id, substance_code, reaction_text,
                   severity_code, status, recorded_at
            FROM allergy_intolerance
            WHERE substance_code = $1
            ORDER BY recorded_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(substance_code)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(allergies)
    }

    pub async fn list_allergies_by_severity(&self, severity_code: &str, limit: i64, offset: i64) -> Result<Vec<AllergyIntolerance>> {
        let allergies = sqlx::query_as::<_, AllergyIntolerance>(
            r#"
            SELECT allergy_id, patient_id, substance_code, reaction_text,
                   severity_code, status, recorded_at
            FROM allergy_intolerance
            WHERE severity_code = $1
            ORDER BY recorded_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(severity_code)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(allergies)
    }

    pub async fn update_allergy(&self, allergy: &AllergyIntolerance) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE allergy_intolerance SET
                substance_code = $2, reaction_text = $3, severity_code = $4,
                status = $5, recorded_at = $6
            WHERE allergy_id = $1
            "#
        )
        .bind(&allergy.allergy_id)
        .bind(&allergy.substance_code)
        .bind(&allergy.reaction_text)
        .bind(&allergy.severity_code)
        .bind(&allergy.status)
        .bind(&allergy.recorded_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_allergy(&self, allergy_id: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE allergy_intolerance SET
                status = 'deleted'
            WHERE allergy_id = $1
            "#
        )
        .bind(allergy_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn count_patient_allergies(&self, patient_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM allergy_intolerance
            WHERE patient_id = $1 AND status != 'deleted'
            "#
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_allergies_by_substance(&self, substance_code: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM allergy_intolerance
            WHERE substance_code = $1
            "#
        )
        .bind(substance_code)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_allergies_by_severity(&self, severity_code: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM allergy_intolerance
            WHERE severity_code = $1
            "#
        )
        .bind(severity_code)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    // Medication Statement CRUD operations
    pub async fn create_medication(&self, medication: &MedicationStatement) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO medication_statement (
                med_stmt_id, patient_id, drug_code, drug_name, dose_text,
                frequency_text, route_code, start_date, end_date, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(&medication.med_stmt_id)
        .bind(&medication.patient_id)
        .bind(&medication.drug_code)
        .bind(&medication.drug_name)
        .bind(&medication.dose_text)
        .bind(&medication.frequency_text)
        .bind(&medication.route_code)
        .bind(&medication.start_date)
        .bind(&medication.end_date)
        .bind(&medication.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_medication(&self, medication_id: &str) -> Result<Option<MedicationStatement>> {
        let medication = sqlx::query_as::<_, MedicationStatement>(
            r#"
            SELECT med_stmt_id, patient_id, drug_code, drug_name, dose_text,
                   frequency_text, route_code, start_date, end_date, status
            FROM medication_statement
            WHERE med_stmt_id = $1
            "#
        )
        .bind(medication_id)
        .fetch_optional(self.db)
        .await?;
        Ok(medication)
    }

    pub async fn list_patient_medications(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<MedicationStatement>> {
        let medications = sqlx::query_as::<_, MedicationStatement>(
            r#"
            SELECT med_stmt_id, patient_id, drug_code, drug_name, dose_text,
                   frequency_text, route_code, start_date, end_date, status
            FROM medication_statement
            WHERE patient_id = $1
            ORDER BY start_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(medications)
    }

    pub async fn list_medications_by_drug(&self, drug_code: &str, limit: i64, offset: i64) -> Result<Vec<MedicationStatement>> {
        let medications = sqlx::query_as::<_, MedicationStatement>(
            r#"
            SELECT med_stmt_id, patient_id, drug_code, drug_name, dose_text,
                   frequency_text, route_code, start_date, end_date, status
            FROM medication_statement
            WHERE drug_code = $1
            ORDER BY start_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(drug_code)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(medications)
    }

    pub async fn list_medications_by_status(&self, status: &str, limit: i64, offset: i64) -> Result<Vec<MedicationStatement>> {
        let medications = sqlx::query_as::<_, MedicationStatement>(
            r#"
            SELECT med_stmt_id, patient_id, drug_code, drug_name, dose_text,
                   frequency_text, route_code, start_date, end_date, status
            FROM medication_statement
            WHERE status = $1
            ORDER BY start_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(medications)
    }

    pub async fn update_medication(&self, medication: &MedicationStatement) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE medication_statement SET
                drug_code = $2, drug_name = $3, dose_text = $4,
                frequency_text = $5, route_code = $6, start_date = $7,
                end_date = $8, status = $9
            WHERE med_stmt_id = $1
            "#
        )
        .bind(&medication.med_stmt_id)
        .bind(&medication.drug_code)
        .bind(&medication.drug_name)
        .bind(&medication.dose_text)
        .bind(&medication.frequency_text)
        .bind(&medication.route_code)
        .bind(&medication.start_date)
        .bind(&medication.end_date)
        .bind(&medication.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn stop_medication(&self, medication_id: &str, end_date: NaiveDate, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE medication_statement SET
                status = 'stopped', end_date = $2
            WHERE med_stmt_id = $1
            "#
        )
        .bind(medication_id)
        .bind(end_date)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn count_patient_medications(&self, patient_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM medication_statement
            WHERE patient_id = $1 AND status != 'deleted'
            "#
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_medications_by_drug(&self, drug_code: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM medication_statement
            WHERE drug_code = $1
            "#
        )
        .bind(drug_code)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_medications_by_status(&self, status: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM medication_statement
            WHERE status = $1
            "#
        )
        .bind(status)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn delete_medication(&self, medication_id: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE medication_statement SET
                status = 'deleted'
            WHERE med_stmt_id = $1
            "#
        )
        .bind(medication_id)
        .execute(self.db)
        .await?;
        Ok(())
    }
}
