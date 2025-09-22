use uuid::Uuid;
use crate::domain::models::{LabTest, Specimen, LabResult, ResultValue};
use crate::domain::repositories::{TestRepo, SpecimenRepo, ResultRepo};

pub struct LabService<'a> {
    pub tests: TestRepo<'a>,
    pub specimens: SpecimenRepo<'a>,
    pub results: ResultRepo<'a>,
}

impl<'a> LabService<'a> {
    pub async fn create_test(&self, req: &crate::http::dto::test_dto::CreateTestReq) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let t = LabTest {
            id,
            code: req.code.clone(),
            name: req.name.clone(),
            specimen_type: req.specimen_type.clone(),
            unit: req.unit.clone(),
            ref_low: req.ref_low,
            ref_high: req.ref_high,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.tests.create(&t).await?;
        Ok(id)
    }

    pub async fn create_specimen(&self, req: &crate::http::dto::specimen_dto::CreateSpecimenReq) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let specimen_no = format!("SPM-{}", &id.to_string()[..8]);
        let s = Specimen {
            id,
            specimen_no,
            order_id: req.order_id,
            patient_id: req.patient_id,
            encounter_id: req.encounter_id,
            specimen_type: req.specimen_type.clone(),
            collected_at: None,
            collected_by: None,
            status: "CREATED".into(),
            note: req.note.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.specimens.create(&s).await?;
        Ok(id)
    }

    pub async fn create_result(&self, specimen_id: Uuid, test_id: Uuid, note: Option<&str>) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let result_no = format!("RES-{}", &id.to_string()[..8]);
        let r = LabResult {
            id,
            result_no,
            specimen_id,
            test_id,
            status: "NEW".into(),
            verified_by: None,
            verified_at: None,
            released_by: None,
            released_at: None,
            note: note.map(|s| s.to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.results.create(&r).await?;
        Ok(id)
    }

    pub async fn enter_values(&self, result_id: Uuid, vals: &[crate::http::dto::result_dto::ResultValueReq]) -> anyhow::Result<()> {
        let items: Vec<ResultValue> = vals.iter().map(|v| ResultValue {
            id: Uuid::new_v4(),
            result_id,
            analyte_code: v.analyte_code.clone(),
            analyte_name: v.analyte_name.clone(),
            value_num: v.value_num,
            value_text: v.value_text.clone(),
            unit: v.unit.clone(),
            ref_low: v.ref_low,
            ref_high: v.ref_high,
            flag: v.flag.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }).collect();
        self.results.enter_values(result_id, &items).await
    }
}
