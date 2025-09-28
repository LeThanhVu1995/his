use anyhow::Result;
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::problem::{Patient, PatientIdentifier, PatientContact, EpisodeOfCare};

pub struct PatientRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> PatientRepo<'a> {
    // Patient CRUD operations
    pub async fn create_patient(&self, patient: &Patient) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO patient (
                patient_id, hospital_id, code, national_id, full_name, date_of_birth,
                gender, phone_number, email, address_line1, address_line2, district,
                city, province, country, status, created_at, created_by, updated_at, updated_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            "#
        )
        .bind(&patient.patient_id)
        .bind(&patient.hospital_id)
        .bind(&patient.code)
        .bind(&patient.national_id)
        .bind(&patient.full_name)
        .bind(&patient.date_of_birth)
        .bind(&patient.gender)
        .bind(&patient.phone_number)
        .bind(&patient.email)
        .bind(&patient.address_line1)
        .bind(&patient.address_line2)
        .bind(&patient.district)
        .bind(&patient.city)
        .bind(&patient.province)
        .bind(&patient.country)
        .bind(&patient.status)
        .bind(&patient.created_at)
        .bind(&patient.created_by)
        .bind(&patient.updated_at)
        .bind(&patient.updated_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_patient(&self, patient_id: &str) -> Result<Option<Patient>> {
        let patient = sqlx::query_as::<_, Patient>(
            r#"
            SELECT patient_id, hospital_id, code, national_id, full_name, date_of_birth,
                   gender, phone_number, email, address_line1, address_line2, district,
                   city, province, country, status, created_at, created_by, updated_at, updated_by
            FROM patient
            WHERE patient_id = $1
            "#
        )
        .bind(patient_id)
        .fetch_optional(self.db)
        .await?;
        Ok(patient)
    }

    pub async fn update_patient(&self, patient: &Patient) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE patient SET
                hospital_id = $2, code = $3, national_id = $4, full_name = $5, date_of_birth = $6,
                gender = $7, phone_number = $8, email = $9, address_line1 = $10, address_line2 = $11,
                district = $12, city = $13, province = $14, country = $15, status = $16,
                updated_at = $17, updated_by = $18
            WHERE patient_id = $1
            "#
        )
        .bind(&patient.patient_id)
        .bind(&patient.hospital_id)
        .bind(&patient.code)
        .bind(&patient.national_id)
        .bind(&patient.full_name)
        .bind(&patient.date_of_birth)
        .bind(&patient.gender)
        .bind(&patient.phone_number)
        .bind(&patient.email)
        .bind(&patient.address_line1)
        .bind(&patient.address_line2)
        .bind(&patient.district)
        .bind(&patient.city)
        .bind(&patient.province)
        .bind(&patient.country)
        .bind(&patient.status)
        .bind(&patient.updated_at)
        .bind(&patient.updated_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_patient(&self, patient_id: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE patient SET status = 'deleted', updated_at = NOW(), updated_by = $2 WHERE patient_id = $1"
        )
        .bind(patient_id)
        .bind(user_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_patients(&self, hospital_id: &str, limit: i64, offset: i64) -> Result<Vec<Patient>> {
        let patients = sqlx::query_as::<_, Patient>(
            r#"
            SELECT patient_id, hospital_id, code, national_id, full_name, date_of_birth,
                   gender, phone_number, email, address_line1, address_line2, district,
                   city, province, country, status, created_at, created_by, updated_at, updated_by
            FROM patient
            WHERE hospital_id = $1 AND status != 'deleted'
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(hospital_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(patients)
    }

    pub async fn search_patients(&self, hospital_id: &str, name: Option<&str>, patient_code: Option<&str>, phone: Option<&str>, email: Option<&str>, limit: i64, offset: i64) -> Result<Vec<Patient>> {
        let patients = sqlx::query_as::<_, Patient>(
            r#"
            SELECT patient_id, hospital_id, code, national_id, full_name, date_of_birth,
                   gender, phone_number, email, address_line1, address_line2, district,
                   city, province, country, status, created_at, created_by, updated_at, updated_by
            FROM patient
            WHERE hospital_id = $1 AND status != 'deleted'
              AND ($2::text IS NULL OR full_name ILIKE '%' || $2 || '%')
              AND ($3::text IS NULL OR code ILIKE '%' || $3 || '%')
              AND ($4::text IS NULL OR phone_number ILIKE '%' || $4 || '%')
              AND ($5::text IS NULL OR email ILIKE '%' || $5 || '%')
            ORDER BY created_at DESC
            LIMIT $6 OFFSET $7
            "#
        )
        .bind(hospital_id)
        .bind(name)
        .bind(patient_code)
        .bind(phone)
        .bind(email)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(patients)
    }

    pub async fn count_patients(&self, hospital_id: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM patient WHERE hospital_id = $1 AND status != 'deleted'"
        )
        .bind(hospital_id)
        .fetch_one(self.db)
        .await?;
        Ok(count.0)
    }

    pub async fn count_search_patients(&self, hospital_id: &str, name: Option<&str>, patient_code: Option<&str>, phone: Option<&str>, email: Option<&str>) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM patient
            WHERE hospital_id = $1 AND status != 'deleted'
              AND ($2::text IS NULL OR full_name ILIKE '%' || $2 || '%')
              AND ($3::text IS NULL OR code ILIKE '%' || $3 || '%')
              AND ($4::text IS NULL OR phone_number ILIKE '%' || $4 || '%')
              AND ($5::text IS NULL OR email ILIKE '%' || $5 || '%')
            "#
        )
        .bind(hospital_id)
        .bind(name)
        .bind(patient_code)
        .bind(phone)
        .bind(email)
        .fetch_one(self.db)
        .await?;
        Ok(count.0)
    }

    // Patient Identifier operations
    pub async fn create_patient_identifier(&self, identifier: &PatientIdentifier) -> Result<()> {
        sqlx::query(
            "INSERT INTO patient_identifier (patient_identifier_id, patient_id, system_code, value, active) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&identifier.patient_identifier_id)
        .bind(&identifier.patient_id)
        .bind(&identifier.system_code)
        .bind(&identifier.value)
        .bind(&identifier.active)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_patient_identifiers(&self, patient_id: &str) -> Result<Vec<PatientIdentifier>> {
        let identifiers = sqlx::query_as::<_, PatientIdentifier>(
            "SELECT patient_identifier_id, patient_id, system_code, value, active FROM patient_identifier WHERE patient_id = $1"
        )
        .bind(patient_id)
        .fetch_all(self.db)
        .await?;
        Ok(identifiers)
    }

    pub async fn update_patient_identifier(&self, identifier: &PatientIdentifier) -> Result<()> {
        sqlx::query(
            "UPDATE patient_identifier SET system_code = $2, value = $3, active = $4 WHERE patient_identifier_id = $1"
        )
        .bind(&identifier.patient_identifier_id)
        .bind(&identifier.system_code)
        .bind(&identifier.value)
        .bind(&identifier.active)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_patient_identifier(&self, identifier_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM patient_identifier WHERE patient_identifier_id = $1")
        .bind(identifier_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    // Patient Contact operations
    pub async fn create_patient_contact(&self, contact: &PatientContact) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO patient_contact (
                patient_contact_id, patient_id, relation_code, name, phone_number,
                email, address_line1, address_line2, city, country, is_primary
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#
        )
        .bind(&contact.patient_contact_id)
        .bind(&contact.patient_id)
        .bind(&contact.relation_code)
        .bind(&contact.name)
        .bind(&contact.phone_number)
        .bind(&contact.email)
        .bind(&contact.address_line1)
        .bind(&contact.address_line2)
        .bind(&contact.city)
        .bind(&contact.country)
        .bind(&contact.is_primary)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_patient_contacts(&self, patient_id: &str) -> Result<Vec<PatientContact>> {
        let contacts = sqlx::query_as::<_, PatientContact>(
            r#"
            SELECT patient_contact_id, patient_id, relation_code, name, phone_number,
                   email, address_line1, address_line2, city, country, is_primary
            FROM patient_contact
            WHERE patient_id = $1
            "#
        )
        .bind(patient_id)
        .fetch_all(self.db)
        .await?;
        Ok(contacts)
    }

    pub async fn update_patient_contact(&self, contact: &PatientContact) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE patient_contact SET
                relation_code = $2, name = $3, phone_number = $4, email = $5,
                address_line1 = $6, address_line2 = $7, city = $8, country = $9, is_primary = $10
            WHERE patient_contact_id = $1
            "#
        )
        .bind(&contact.patient_contact_id)
        .bind(&contact.relation_code)
        .bind(&contact.name)
        .bind(&contact.phone_number)
        .bind(&contact.email)
        .bind(&contact.address_line1)
        .bind(&contact.address_line2)
        .bind(&contact.city)
        .bind(&contact.country)
        .bind(&contact.is_primary)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_patient_contact(&self, contact_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM patient_contact WHERE patient_contact_id = $1")
        .bind(contact_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    // Episode of Care operations
    pub async fn create_episode(&self, episode: &EpisodeOfCare) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO episode_of_care (
                episode_id, patient_id, status, start_date, end_date, reason_text
            ) VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(&episode.episode_id)
        .bind(&episode.patient_id)
        .bind(&episode.status)
        .bind(&episode.start_date)
        .bind(&episode.end_date)
        .bind(&episode.reason_text)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_episode(&self, episode_id: &str) -> Result<Option<EpisodeOfCare>> {
        let episode = sqlx::query_as::<_, EpisodeOfCare>(
            "SELECT episode_id, patient_id, status, start_date, end_date, reason_text FROM episode_of_care WHERE episode_id = $1"
        )
        .bind(episode_id)
        .fetch_optional(self.db)
        .await?;
        Ok(episode)
    }

    pub async fn get_patient_episodes(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<EpisodeOfCare>> {
        let episodes = sqlx::query_as::<_, EpisodeOfCare>(
            "SELECT episode_id, patient_id, status, start_date, end_date, reason_text FROM episode_of_care WHERE patient_id = $1 ORDER BY start_date DESC LIMIT $2 OFFSET $3"
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(episodes)
    }

    pub async fn update_episode(&self, episode: &EpisodeOfCare) -> Result<()> {
        sqlx::query(
            "UPDATE episode_of_care SET status = $2, start_date = $3, end_date = $4, reason_text = $5 WHERE episode_id = $1"
        )
        .bind(&episode.episode_id)
        .bind(&episode.status)
        .bind(&episode.start_date)
        .bind(&episode.end_date)
        .bind(&episode.reason_text)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn close_episode(&self, episode_id: &str, end_date: chrono::DateTime<chrono::Utc>, _user_id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE episode_of_care SET status = 'finished', end_date = $2 WHERE episode_id = $1"
        )
        .bind(episode_id)
        .bind(end_date)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn count_patient_episodes(&self, patient_id: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM episode_of_care WHERE patient_id = $1"
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count.0)
    }

    pub async fn get_patient_by_code(&self, code: &str) -> Result<Option<Patient>> {
        let patient = sqlx::query_as::<_, Patient>(
            r#"
            SELECT patient_id, hospital_id, code, national_id, full_name, date_of_birth,
                   gender, phone_number, email, address_line1, address_line2, district,
                   city, province, country, status, created_at, created_by, updated_at, updated_by
            FROM patient
            WHERE code = $1 AND status != 'deleted'
            "#
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?;
        Ok(patient)
    }
}
