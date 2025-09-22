use uuid::Uuid;
use crate::infrastructure::repositories::claim_repo::ClaimRepo;
use crate::domain::entities::claim::{Claim, ClaimItem};

pub struct ClaimSvc<'a> {
    pub repo: ClaimRepo<'a>,
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> ClaimSvc<'a> {
    pub async fn create_with_items(
        &self,
        patient: Uuid,
        member: Uuid,
        payer: &str,
        items: Vec<(String, f64, f64, Option<String>)>,
        encounter: Option<Uuid>,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let mut tx = self.db.begin().await?;
        let mut total = 0.0;

        for (_, qty, unit, _) in &items {
            total += qty * unit;
        }

        let claim = Claim {
            id,
            claim_no: format!("CLM-{}", &id.to_string()[..8]),
            patient_id: patient,
            encounter_id: encounter,
            member_id: member,
            payer: payer.to_string(),
            total_amount: total,
            currency: "VND".into(),
            status: "CREATED".into(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        sqlx::query(
            r#"INSERT INTO claims(id,claim_no,patient_id,encounter_id,member_id,payer,total_amount,currency,status) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)"#
        )
        .bind(claim.id)
        .bind(&claim.claim_no)
        .bind(claim.patient_id)
        .bind(claim.encounter_id)
        .bind(claim.member_id)
        .bind(&claim.payer)
        .bind(claim.total_amount)
        .bind(&claim.currency)
        .bind(&claim.status)
        .execute(&mut *tx)
        .await?;

        for (code, qty, unit_price, desc) in items {
            let it = ClaimItem {
                id: Uuid::new_v4(),
                claim_id: id,
                code,
                description: desc,
                qty,
                unit_price,
                amount: qty * unit_price,
                coverage_rate: Some(100.0),
                patient_pay: Some(0.0),
            };

            sqlx::query(
                r#"INSERT INTO claim_items(id,claim_id,code,description,qty,unit_price,coverage_rate,patient_pay) VALUES($1,$2,$3,$4,$5,$6,$7,$8)"#
            )
            .bind(it.id)
            .bind(it.claim_id)
            .bind(&it.code)
            .bind(&it.description)
            .bind(it.qty)
            .bind(it.unit_price)
            .bind(it.coverage_rate)
            .bind(it.patient_pay)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(id)
    }

    pub async fn submit(&self, id: Uuid) -> anyhow::Result<()> {
        self.repo.update_status(id, "SUBMITTED").await
    }

    pub async fn sign(&self, id: Uuid) -> anyhow::Result<()> {
        self.repo.update_status(id, "SIGNED").await
    }

    pub async fn set_status(&self, id: Uuid, st: &str) -> anyhow::Result<()> {
        self.repo.update_status(id, st).await
    }
}
