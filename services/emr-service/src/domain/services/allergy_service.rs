use anyhow::Result;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::allergy::{AllergyIntolerance, MedicationStatement};
use crate::infra::db::repositories::AllergyRepo;
use crate::http::dto::allergy::*;

pub struct AllergyService<'a> {
    repo: AllergyRepo<'a>,
}

impl<'a> AllergyService<'a> {
    pub fn new(repo: AllergyRepo<'a>) -> Self {
        Self { repo }
    }

    // Allergy Intolerance CRUD operations
    pub async fn create_allergy(&self, req: CreateAllergyRequest) -> Result<AllergyIntolerance> {
        let allergy_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let allergy = AllergyIntolerance {
            allergy_id: allergy_id.clone(),
            patient_id: req.patient_id,
            substance_code: req.allergen_code,
            reaction_text: req.description,
            severity_code: req.severity,
            status: "active".to_string(),
            recorded_at: now,
        };

        self.repo.create_allergy(&allergy).await?;
        Ok(allergy)
    }

    pub async fn get_allergy(&self, allergy_id: &str) -> Result<Option<AllergyIntolerance>> {
        self.repo.get_allergy(allergy_id).await
    }

    pub async fn list_patient_allergies(
        &self,
        patient_id: &str,
        category: Option<&str>,
        severity: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<AllergyIntolerance>, i64)> {
        let allergies = self.repo.list_patient_allergies(patient_id, limit, offset).await?;
        let total = self.repo.count_patient_allergies(patient_id).await?;
        Ok((allergies, total))
    }

    pub async fn update_allergy(&self, allergy_id: &str, req: UpdateAllergyRequest) -> Result<AllergyIntolerance> {
        let mut allergy = self.repo.get_allergy(allergy_id).await?
            .ok_or_else(|| anyhow::anyhow!("Allergy not found"))?;

        if let Some(allergen_code) = req.allergen_code {
            allergy.substance_code = allergen_code;
        }
        if let Some(severity) = req.severity {
            allergy.severity_code = Some(severity);
        }
        if let Some(description) = req.description {
            allergy.reaction_text = Some(description);
        }

        self.repo.update_allergy(&allergy).await?;
        Ok(allergy)
    }

    pub async fn delete_allergy(&self, allergy_id: &str, user_id: &str) -> Result<()> {
        self.repo.delete_allergy(allergy_id, user_id).await
    }

    // Medication Statement CRUD operations
    pub async fn create_medication(&self, req: CreateMedicationRequest) -> Result<MedicationStatement> {
        let medication_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let medication = MedicationStatement {
            med_stmt_id: medication_id.clone(),
            patient_id: req.patient_id,
            drug_code: req.medication_code,
            drug_name: req.medication_name,
            dose_text: req.dosage,
            frequency_text: req.frequency,
            route_code: req.route,
            start_date: req.start_date,
            end_date: req.end_date,
            status: req.status.unwrap_or_else(|| "active".to_string()),
        };

        self.repo.create_medication(&medication).await?;
        Ok(medication)
    }

    pub async fn get_medication(&self, medication_id: &str) -> Result<Option<MedicationStatement>> {
        self.repo.get_medication(medication_id).await
    }

    pub async fn list_patient_medications(
        &self,
        patient_id: &str,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<MedicationStatement>, i64)> {
        let medications = self.repo.list_patient_medications(patient_id, limit, offset).await?;
        let total = self.repo.count_patient_medications(patient_id).await?;
        Ok((medications, total))
    }

    pub async fn update_medication(&self, medication_id: &str, req: UpdateMedicationRequest) -> Result<MedicationStatement> {
        let mut medication = self.repo.get_medication(medication_id).await?
            .ok_or_else(|| anyhow::anyhow!("Medication not found"))?;

        if let Some(medication_code) = req.medication_code {
            medication.drug_code = medication_code;
        }
        if let Some(medication_name) = req.medication_name {
            medication.drug_name = medication_name;
        }
        if let Some(status) = req.status {
            medication.status = status;
        }
        if let Some(dosage) = req.dosage {
            medication.dose_text = Some(dosage);
        }
        if let Some(frequency) = req.frequency {
            medication.frequency_text = Some(frequency);
        }
        if let Some(route) = req.route {
            medication.route_code = Some(route);
        }
        if let Some(start_date) = req.start_date {
            medication.start_date = Some(start_date);
        }
        if let Some(end_date) = req.end_date {
            medication.end_date = Some(end_date);
        }

        self.repo.update_medication(&medication).await?;
        Ok(medication)
    }

    pub async fn delete_medication(&self, medication_id: &str, user_id: &str) -> Result<()> {
        self.repo.delete_medication(medication_id, user_id).await
    }
}
