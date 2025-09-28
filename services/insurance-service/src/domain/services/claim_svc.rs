use uuid::Uuid;
use crate::infrastructure::repositories::claim_repo::ClaimRepo;
use crate::domain::entities::claim::{InsClaim, InsClaimItem};

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

        let claim = InsClaim {
            claim_id: id.to_string(),
            encounter_id: encounter.map(|e| e.to_string()).unwrap_or_default(),
            policy_id: member.to_string(), // Using member as policy_id for now
            status: "CREATED".into(),
            total_claimed: Some(total),
            total_approved: None,
            submitted_at: None,
            response_at: None,
            response_code: None,
            response_text: None,
            signature_id: None,
        };

        sqlx::query(
            r#"INSERT INTO ins_claim(claim_id,encounter_id,policy_id,status,total_claimed) VALUES($1,$2,$3,$4,$5)"#
        )
        .bind(&claim.claim_id)
        .bind(&claim.encounter_id)
        .bind(&claim.policy_id)
        .bind(&claim.status)
        .bind(&claim.total_claimed)
        .execute(&mut *tx)
        .await?;

        for (code, qty, unit_price, desc) in items {
            let it = InsClaimItem {
                claim_item_id: Uuid::new_v4().to_string(),
                claim_id: id.to_string(),
                service_code: code,
                description: desc,
                qty: Some(qty),
                unit_price: Some(unit_price),
                amount: Some(qty * unit_price),
                approved_amount: None,
            };

            sqlx::query(
                r#"INSERT INTO ins_claim_item(claim_item_id,claim_id,service_code,description,qty,unit_price,amount) VALUES($1,$2,$3,$4,$5,$6,$7)"#
            )
            .bind(&it.claim_item_id)
            .bind(&it.claim_id)
            .bind(&it.service_code)
            .bind(&it.description)
            .bind(&it.qty)
            .bind(&it.unit_price)
            .bind(&it.amount)
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
