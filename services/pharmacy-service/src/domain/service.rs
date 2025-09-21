use uuid::Uuid;
// use rust_decimal::Decimal; // Not supported by SQLx
use crate::domain::models::{Medication, Prescription, PrescriptionItem, Dispense};
use crate::domain::repositories::{MedRepo, PrescriptionRepo, PrescriptionItemRepo, DispenseRepo};

pub struct PharmacyService<'a> {
    pub meds: MedRepo<'a>,
    pub presc: PrescriptionRepo<'a>,
    pub items: PrescriptionItemRepo<'a>,
    pub disp: DispenseRepo<'a>,
}

impl<'a> PharmacyService<'a> {
    pub async fn create_med(&self, req: &crate::http::dto::medication_dto::CreateMedicationReq) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let m = Medication {
            id,
            code: req.code.clone(),
            name: req.name.clone(),
            strength: req.strength.clone(),
            form: req.form.clone(),
            route: req.route.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.meds.create(&m).await?;
        Ok(id)
    }

    pub async fn create_prescription(&self, req: &crate::http::dto::prescription_dto::CreatePrescriptionReq, ordered_by: Option<&str>) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let presc_no = format!("PRC-{}", &id.to_string()[..8]);
        let p = Prescription {
            id,
            patient_id: req.patient_id,
            encounter_id: req.encounter_id,
            presc_no,
            status: "NEW".into(),
            ordered_by: ordered_by.map(|s| s.to_string()),
            note: req.note.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.presc.create(&p).await?;
        let mut items = Vec::new();
        for it in &req.items {
            items.push(PrescriptionItem {
                id: Uuid::new_v4(),
                prescription_id: id,
                medication_id: it.medication_id,
                dose: it.dose.clone(),
                freq: it.freq.clone(),
                duration: it.duration.clone(),
                qty: it.qty,
                instruction: it.instruction.clone(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            });
        }
        self.items.insert_many(&items).await?;
        Ok(id)
    }

    pub async fn create_dispense(&self, presc_id: Uuid, dispensed_by: Option<&str>) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let disp_no = format!("DSP-{}", &id.to_string()[..8]);
        let d = Dispense {
            id,
            prescription_id: presc_id,
            disp_no,
            dispensed_by: dispensed_by.map(|s| s.to_string()),
            status: "NEW".into(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.disp.create(&d).await?;
        Ok(id)
    }
}
