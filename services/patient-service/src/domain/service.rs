use crate::domain::{models::{Patient, Encounter}, repo::{PatientRepo, EncounterRepo}};
use crate::infrastructure::kafka::Kafka;
use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
struct PatientEvent<'a, T> {
    event: &'a str,
    data: T
}

#[derive(Serialize)]
struct EncounterEvent<'a, T> {
    event: &'a str,
    data: T
}

pub struct PatientService<'a> {
    pub repo: PatientRepo<'a>,
    pub kafka: Option<&'a Kafka>
}

impl<'a> PatientService<'a> {
    pub async fn create(&self, req: &crate::dto::patient_dto::CreatePatientReq) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let p = Patient {
            id,
            mrn: req.mrn.clone(),
            national_id: req.national_id.clone(),
            passport_no: req.passport_no.clone(),
            full_name: req.full_name.clone(),
            first_name: None,
            last_name: None,
            gender: req.gender.clone(),
            birth_date: req.birth_date,
            phone: req.phone.clone(),
            email: req.email.clone(),
            address: req.address.clone(),
            blood_type: None,
            marital_status: None,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now()
        };
        self.repo.create(&p).await?;
        if let Some(k) = self.kafka {
            let evt = PatientEvent { event: "his.patient.created", data: &p };
            k.publish("his.patient.v1", &id.to_string(), &evt).await.ok();
        }
        Ok(id)
    }
}

pub struct EncounterService<'a> {
    pub repo: EncounterRepo<'a>,
    pub kafka: Option<&'a Kafka>
}

impl<'a> EncounterService<'a> {
    pub async fn create(&self, req: &crate::dto::encounter_dto::CreateEncounterReq) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let e = Encounter {
            id,
            patient_id: req.patient_id,
            encounter_no: req.encounter_no.clone(),
            encounter_type: req.encounter_type.clone(),
            status: "PLANNED".into(),
            department_code: req.department_code.clone(),
            attending_doctor_id: None,
            admitted_at: Some(chrono::Utc::now()),
            discharged_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now()
        };
        self.repo.create(&e).await?;
        if let Some(k) = self.kafka {
            let evt = EncounterEvent { event: "his.encounter.created", data: &e };
            k.publish("his.encounter.v1", &id.to_string(), &evt).await.ok();
        }
        Ok(id)
    }
}
