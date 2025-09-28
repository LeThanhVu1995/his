use anyhow::Result;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::problem::{Patient, PatientIdentifier, PatientContact, EpisodeOfCare};
use crate::infra::db::repositories::PatientRepo;
use crate::http::dto::patient::*;

pub struct PatientService<'a> {
    repo: PatientRepo<'a>,
}

impl<'a> PatientService<'a> {
    pub fn new(repo: PatientRepo<'a>) -> Self {
        Self { repo }
    }

    // Patient CRUD operations
    pub async fn create_patient(&self, req: CreatePatientRequest) -> Result<Patient> {
        let patient_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let patient = Patient {
            patient_id: patient_id.clone(),
            hospital_id: req.hospital_id,
            code: req.code,
            national_id: req.national_id,
            full_name: req.full_name,
            date_of_birth: req.date_of_birth,
            gender: req.gender,
            phone_number: req.phone_number,
            email: req.email,
            address_line1: req.address_line1,
            address_line2: req.address_line2,
            district: req.district,
            city: req.city,
            province: req.province,
            country: req.country,
            status: "active".to_string(),
            created_at: now,
            created_by: Some("system".to_string()),
            updated_at: now,
            updated_by: Some("system".to_string()),
            deleted_at: None,
            deleted_by: None,
        };

        self.repo.create_patient(&patient).await?;
        Ok(patient)
    }

    pub async fn get_patient(&self, patient_id: &str) -> Result<Option<Patient>> {
        self.repo.get_patient(patient_id).await
    }

    pub async fn get_patient_by_code(&self, code: &str) -> Result<Option<Patient>> {
        self.repo.get_patient_by_code(code).await
    }

    pub async fn list_patients(&self, limit: i64, offset: i64) -> Result<(Vec<Patient>, i64)> {
        let patients = self.repo.list_patients("", limit, offset).await?;
        let total = self.repo.count_patients("").await?;
        Ok((patients, total))
    }

    pub async fn update_patient(&self, patient_id: &str, req: UpdatePatientRequest) -> Result<Patient> {
        let mut patient = self.repo.get_patient(patient_id).await?
            .ok_or_else(|| anyhow::anyhow!("Patient not found"))?;

        if let Some(code) = req.code {
            patient.code = Some(code);
        }
        if let Some(national_id) = req.national_id {
            patient.national_id = Some(national_id);
        }
        if let Some(full_name) = req.full_name {
            patient.full_name = full_name;
        }
        if let Some(date_of_birth) = req.date_of_birth {
            patient.date_of_birth = Some(date_of_birth);
        }
        if let Some(gender) = req.gender {
            patient.gender = Some(gender);
        }
        if let Some(phone_number) = req.phone_number {
            patient.phone_number = Some(phone_number);
        }
        if let Some(email) = req.email {
            patient.email = Some(email);
        }
        if let Some(address_line1) = req.address_line1 {
            patient.address_line1 = Some(address_line1);
        }
        if let Some(address_line2) = req.address_line2 {
            patient.address_line2 = Some(address_line2);
        }
        if let Some(district) = req.district {
            patient.district = Some(district);
        }
        if let Some(city) = req.city {
            patient.city = Some(city);
        }
        if let Some(province) = req.province {
            patient.province = Some(province);
        }
        if let Some(country) = req.country {
            patient.country = Some(country);
        }
        if let Some(status) = req.status {
            patient.status = status;
        }

        patient.updated_at = Utc::now();
        patient.updated_by = Some("system".to_string());

        self.repo.update_patient(&patient).await?;
        Ok(patient)
    }

    pub async fn delete_patient(&self, patient_id: &str, user_id: &str) -> Result<()> {
        self.repo.delete_patient(patient_id, user_id).await
    }

    // Patient Identifier operations
    pub async fn create_patient_identifier(&self, req: CreatePatientIdentifierRequest) -> Result<PatientIdentifier> {
        let identifier_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let identifier = PatientIdentifier {
            patient_identifier_id: identifier_id,
            patient_id: req.patient_id,
            system_code: req.system_code,
            value: req.value,
            active: req.active,
        };

        self.repo.create_patient_identifier(&identifier).await?;
        Ok(identifier)
    }

    pub async fn get_patient_identifiers(&self, patient_id: &str) -> Result<Vec<PatientIdentifier>> {
        self.repo.get_patient_identifiers(patient_id).await
    }

    // Patient Contact operations
    pub async fn create_patient_contact(&self, req: CreatePatientContactRequest) -> Result<PatientContact> {
        let contact_id = Uuid::new_v4().to_string();

        let contact = PatientContact {
            patient_contact_id: contact_id,
            patient_id: req.patient_id,
            relation_code: req.relation_code,
            name: req.name,
            phone_number: req.phone_number,
            email: req.email,
            address_line1: req.address_line1,
            address_line2: req.address_line2,
            city: req.city,
            country: req.country,
            is_primary: None,
        };

        self.repo.create_patient_contact(&contact).await?;
        Ok(contact)
    }

    pub async fn get_patient_contacts(&self, patient_id: &str) -> Result<Vec<PatientContact>> {
        self.repo.get_patient_contacts(patient_id).await
    }

    // Episode of Care operations
    pub async fn create_episode(&self, req: CreateEpisodeRequest) -> Result<EpisodeOfCare> {
        let episode_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let episode = EpisodeOfCare {
            episode_id,
            patient_id: req.patient_id,
        start_date: req.start_date.unwrap_or_else(|| Utc::now().date_naive()).and_hms_opt(0, 0, 0).unwrap().and_utc(),
        end_date: req.end_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc()),
            status: "active".to_string(),
            reason_text: req.description,
        };

        self.repo.create_episode(&episode).await?;
        Ok(episode)
    }

    pub async fn get_patient_episodes(&self, patient_id: &str) -> Result<Vec<EpisodeOfCare>> {
        self.repo.get_patient_episodes(patient_id, 100, 0).await
    }

    pub async fn close_episode(&self, episode_id: &str, end_date: NaiveDate) -> Result<EpisodeOfCare> {
        let mut episode = self.repo.get_episode(episode_id).await?
            .ok_or_else(|| anyhow::anyhow!("Episode not found"))?;

        episode.status = "closed".to_string();
        episode.end_date = Some(end_date.and_hms_opt(0, 0, 0).unwrap().and_utc());

        self.repo.update_episode(&episode).await?;
        Ok(episode)
    }

    // Search operations
    pub async fn search_patients(&self, req: SearchPatientRequest) -> Result<(Vec<Patient>, i64)> {
        let patients = self.repo.search_patients(
            &req.hospital_id,
            req.name.as_deref(),
            req.patient_code.as_deref(),
            req.phone.as_deref(),
            req.email.as_deref(),
            req.limit.unwrap_or(50),
            req.offset.unwrap_or(0),
        ).await?;

        let total = self.repo.count_search_patients(
            &req.hospital_id,
            req.name.as_deref(),
            req.patient_code.as_deref(),
            req.phone.as_deref(),
            req.email.as_deref(),
        ).await?;

        Ok((patients, total))
    }
}
