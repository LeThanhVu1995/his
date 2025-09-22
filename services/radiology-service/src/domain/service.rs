use uuid::Uuid;
use crate::domain::models::*;
use crate::domain::repo::{ProcRepo, OrderRepo, StudyRepo, ReportRepo};

pub struct RisService<'a> {
    pub procs: ProcRepo<'a>,
    pub orders: OrderRepo<'a>,
    pub studies: StudyRepo<'a>,
    pub reports: ReportRepo<'a>,
}

impl<'a> RisService<'a> {
    pub async fn create_procedure(&self, req: &crate::dto::procedure_dto::CreateProcedureReq) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let p = Procedure {
            id,
            code: req.code.clone(),
            name: req.name.clone(),
            modality: req.modality.clone(),
            body_part: req.body_part.clone(),
            contrast: req.contrast.unwrap_or(false),
            duration_min: req.duration_min,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.procs.create(&p).await?;
        Ok(id)
    }

    pub async fn create_order(&self, req: &crate::dto::order_dto::CreateOrderReq, requested_by: Option<&str>) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let order_no = format!("ORD-{}", &id.to_string()[..8]);
        let o = RadOrder {
            id,
            order_no,
            patient_id: req.patient_id,
            encounter_id: req.encounter_id,
            procedure_id: req.procedure_id,
            reason: req.reason.clone(),
            priority: req.priority.clone().unwrap_or_else(|| "ROUTINE".into()),
            status: "NEW".into(),
            requested_by: requested_by.map(|s| s.to_string()),
            scheduled_at: req.scheduled_at,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.orders.create(&o).await?;
        Ok(id)
    }

    pub async fn create_study(&self, order_id: Uuid, modality: &str) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let s = Study {
            id,
            study_uid: Uuid::new_v4(),
            order_id,
            accession_no: format!("ACC-{}", &id.to_string()[..8]),
            modality: modality.into(),
            started_at: None,
            ended_at: None,
            performer: None,
            status: "CREATED".into(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.studies.create(&s).await?;
        Ok(id)
    }

    pub async fn create_report(&self, study_id: Uuid) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let r = Report {
            id,
            report_no: format!("REP-{}", &id.to_string()[..8]),
            study_id,
            status: "DRAFT".into(),
            content: None,
            author: None,
            verified_by: None,
            verified_at: None,
            finalized_by: None,
            finalized_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.reports.create(&r).await?;
        Ok(id)
    }
}
