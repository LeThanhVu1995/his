use anyhow::Result;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::vital::{VitalSignRecord, VitalSignItem, Observation};
use crate::infra::db::repositories::VitalRepo;
use crate::http::dto::vital::*;

pub struct VitalService<'a> {
    repo: VitalRepo<'a>,
}

impl<'a> VitalService<'a> {
    pub fn new(repo: VitalRepo<'a>) -> Self {
        Self { repo }
    }

    // Vital Sign Record CRUD operations
    pub async fn create_vital_sign_record(&self, req: CreateVitalSignRecordRequest) -> Result<VitalSignRecord> {
        let record_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let record = VitalSignRecord {
            vs_id: record_id.clone(),
            encounter_id: req.encounter_id,
            patient_id: req.patient_id,
            measured_at: req.recorded_at.map(|dt| dt.and_utc()).unwrap_or_else(|| Utc::now()),
            recorder_staff_id: req.created_by,
            note: req.notes,
        };

        self.repo.create_vital_record(&record).await?;
        Ok(record)
    }

    pub async fn get_vital_sign_record(&self, record_id: &str) -> Result<Option<VitalSignRecord>> {
        self.repo.get_vital_record(record_id).await
    }

    pub async fn list_patient_vital_signs(
        &self,
        patient_id: &str,
        record_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VitalSignRecord>, i64)> {
        let records = self.repo.list_patient_vital_records(patient_id, limit, offset).await?;
        let total = self.repo.count_patient_vital_records(patient_id).await?;
        Ok((records, total))
    }

    pub async fn update_vital_sign_record(&self, record_id: &str, req: UpdateVitalSignRecordRequest) -> Result<VitalSignRecord> {
        let mut record = self.repo.get_vital_record(record_id).await?
            .ok_or_else(|| anyhow::anyhow!("Vital sign record not found"))?;

        if let Some(recorded_at) = req.recorded_at {
            record.measured_at = recorded_at.and_utc();
        }
        if let Some(notes) = req.notes {
            record.note = Some(notes);
        }

        self.repo.update_vital_record(&record).await?;
        Ok(record)
    }

    pub async fn delete_vital_sign_record(&self, record_id: &str, user_id: &str) -> Result<()> {
        self.repo.delete_vital_record(record_id, user_id).await
    }

    // Vital Sign Item CRUD operations
    pub async fn create_vital_sign_item(&self, req: CreateVitalSignItemRequest) -> Result<VitalSignItem> {
        let item_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let item = VitalSignItem {
            vs_item_id: item_id.clone(),
            vs_id: req.record_id,
            code: req.vital_type,
            value_num: None,
            value_text: Some(req.value),
            unit: req.unit,
        };

        self.repo.create_vital_item(&item).await?;
        Ok(item)
    }

    pub async fn get_vital_sign_items(&self, record_id: &str) -> Result<Vec<VitalSignItem>> {
        self.repo.get_vital_items(record_id).await
    }

    pub async fn update_vital_sign_item(&self, item_id: &str, req: UpdateVitalSignItemRequest) -> Result<VitalSignItem> {
        let mut item = self.repo.get_vital_item(item_id).await?
            .ok_or_else(|| anyhow::anyhow!("Vital sign item not found"))?;

        if let Some(vital_type) = req.vital_type {
            item.code = vital_type;
        }
        if let Some(value) = req.value {
            item.value_text = Some(value);
        }
        if let Some(unit) = req.unit {
            item.unit = Some(unit);
        }

        self.repo.update_vital_item(&item).await?;
        Ok(item)
    }

    pub async fn delete_vital_sign_item(&self, item_id: &str, user_id: &str) -> Result<()> {
        self.repo.delete_vital_item(item_id).await
    }

    // Observation CRUD operations
    pub async fn create_observation(&self, req: CreateObservationRequest) -> Result<Observation> {
        let observation_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let observation = Observation {
            obs_id: observation_id.clone(),
            encounter_id: req.encounter_id,
            patient_id: req.patient_id,
            code: req.observation_type,
            value_num: None,
            value_text: Some(req.value),
            unit: req.unit,
            taken_at: req.observed_at.map(|dt| dt.and_utc()).unwrap_or_else(|| Utc::now()),
            performer_staff_id: req.created_by,
            status: "final".to_string(),
        };

        self.repo.create_observation(&observation).await?;
        Ok(observation)
    }

    pub async fn get_observation(&self, observation_id: &str) -> Result<Option<Observation>> {
        self.repo.get_observation(observation_id).await
    }

    pub async fn list_patient_observations(
        &self,
        patient_id: &str,
        observation_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Observation>, i64)> {
        let observations = self.repo.list_patient_observations(patient_id, limit, offset).await?;
        let total = self.repo.count_patient_observations(patient_id).await?;
        Ok((observations, total))
    }

    pub async fn update_observation(&self, observation_id: &str, req: UpdateObservationRequest) -> Result<Observation> {
        let mut observation = self.repo.get_observation(observation_id).await?
            .ok_or_else(|| anyhow::anyhow!("Observation not found"))?;

        if let Some(observation_type) = req.observation_type {
            observation.code = observation_type;
        }
        if let Some(value) = req.value {
            observation.value_text = Some(value);
        }
        if let Some(unit) = req.unit {
            observation.unit = Some(unit);
        }
        if let Some(observed_at) = req.observed_at {
            observation.taken_at = observed_at.and_utc();
        }

        self.repo.update_observation(&observation).await?;
        Ok(observation)
    }

    pub async fn delete_observation(&self, observation_id: &str, user_id: &str) -> Result<()> {
        self.repo.delete_observation(observation_id, user_id).await
    }
}
