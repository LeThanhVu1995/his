use anyhow::Result;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::problem::{Encounter, ClinicalNote};
use crate::infra::db::repositories::{EncounterRepo, PatientRepo};
use crate::http::dto::encounter::*;

pub struct EncounterService<'a> {
    encounter_repo: EncounterRepo<'a>,
    patient_repo: PatientRepo<'a>,
}

impl<'a> EncounterService<'a> {
    pub fn new(encounter_repo: EncounterRepo<'a>, patient_repo: PatientRepo<'a>) -> Self {
        Self { encounter_repo, patient_repo }
    }

    // Encounter CRUD operations
    pub async fn create_encounter(&self, req: CreateEncounterRequest) -> Result<Encounter> {
        // Verify patient exists
        self.patient_repo.get_patient(&req.patient_id).await?
            .ok_or_else(|| anyhow::anyhow!("Patient not found"))?;

        let encounter_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let encounter = Encounter {
            encounter_id: encounter_id.clone(),
            patient_id: req.patient_id,
            episode_id: None,
            facility_id: req.facility_id,
            department_id: None,
            room_id: None,
            bed_id: None,
            type_code: req.encounter_type,
            start_time: req.start_time.map(|dt| dt.and_utc()).unwrap_or_else(|| Utc::now()),
            end_time: req.end_time.map(|dt| dt.and_utc()),
            status: req.status.unwrap_or_else(|| "active".to_string()),
            attending_staff_id: None, // Not available in request
        };

        self.encounter_repo.create_encounter(&encounter).await?;
        Ok(encounter)
    }

    pub async fn get_encounter(&self, encounter_id: &str) -> Result<Option<Encounter>> {
        self.encounter_repo.get_encounter(encounter_id).await
    }

    pub async fn list_patient_encounters(
        &self,
        patient_id: &str,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Encounter>, i64)> {
        let encounters = self.encounter_repo.list_patient_encounters(patient_id, limit, offset).await?;
        let total = self.encounter_repo.count_patient_encounters(patient_id).await?;
        Ok((encounters, total))
    }

    pub async fn list_encounters_by_facility(
        &self,
        facility_id: &str,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Encounter>, i64)> {
        let encounters = self.encounter_repo.list_encounters_by_facility(facility_id, limit, offset).await?;
        let total = self.encounter_repo.count_encounters_by_facility(facility_id).await?;
        Ok((encounters, total))
    }

    pub async fn update_encounter(&self, encounter_id: &str, req: UpdateEncounterRequest) -> Result<Encounter> {
        let mut encounter = self.encounter_repo.get_encounter(encounter_id).await?
            .ok_or_else(|| anyhow::anyhow!("Encounter not found"))?;

        if let Some(status) = req.status {
            encounter.status = status;
        }
        if let Some(start_time) = req.start_time {
            encounter.start_time = start_time.and_utc();
        }
        if let Some(end_time) = req.end_time {
            encounter.end_time = Some(end_time.and_utc());
        }

        self.encounter_repo.update_encounter(&encounter).await?;
        Ok(encounter)
    }

    pub async fn end_encounter(&self, encounter_id: &str, user_id: &str) -> Result<()> {
        let mut encounter = self.encounter_repo.get_encounter(encounter_id).await?
            .ok_or_else(|| anyhow::anyhow!("Encounter not found"))?;

        encounter.status = "ended".to_string();
        encounter.end_time = Some(Utc::now());

        self.encounter_repo.update_encounter(&encounter).await?;
        Ok(())
    }

    // Clinical Note operations
    pub async fn create_clinical_note(&self, req: CreateClinicalNoteRequest) -> Result<ClinicalNote> {
        // Verify encounter exists
        self.encounter_repo.get_encounter(&req.encounter_id).await?
            .ok_or_else(|| anyhow::anyhow!("Encounter not found"))?;

        let note_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let note = ClinicalNote {
            note_id: note_id.clone(),
            encounter_id: req.encounter_id,
            author_staff_id: req.created_by,
            category_code: Some(req.note_type),
            content_text: Some(req.content),
            created_at: now,
        };

        self.encounter_repo.create_clinical_note(&note).await?;
        Ok(note)
    }

    pub async fn list_encounter_notes(
        &self,
        encounter_id: &str,
        note_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<ClinicalNote>, i64)> {
        let notes = self.encounter_repo.list_encounter_notes(encounter_id, limit, offset).await?;
        let total = self.encounter_repo.count_encounter_notes(encounter_id).await?;
        Ok((notes, total))
    }

    pub async fn update_clinical_note(&self, note_id: &str, req: UpdateClinicalNoteRequest) -> Result<ClinicalNote> {
        let mut note = self.encounter_repo.get_clinical_note(note_id).await?
            .ok_or_else(|| anyhow::anyhow!("Clinical note not found"))?;

        if let Some(note_type) = req.note_type {
            note.category_code = Some(note_type);
        }
        if let Some(content) = req.content {
            note.content_text = Some(content);
        }

        self.encounter_repo.update_clinical_note(&note).await?;
        Ok(note)
    }

    pub async fn delete_clinical_note(&self, note_id: &str, user_id: &str) -> Result<()> {
        self.encounter_repo.delete_clinical_note(note_id, user_id).await
    }
}
