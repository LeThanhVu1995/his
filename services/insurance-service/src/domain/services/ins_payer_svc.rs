use anyhow::Result;
use crate::domain::entities::ins_payer::InsPayer;
use crate::infrastructure::repositories::ins_payer_repo::InsPayerRepo;
use sqlx::PgPool;

pub struct InsPayerSvc<'a> {
    pub repo: InsPayerRepo<'a>,
}

impl<'a> InsPayerSvc<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self {
            repo: InsPayerRepo::new(db),
        }
    }

    pub async fn create_payer(&self, code: &str, name: &str) -> Result<InsPayer> {
        let payer = InsPayer {
            payer_id: uuid::Uuid::new_v4().to_string(),
            code: code.to_string(),
            name: name.to_string(),
        };

        self.repo.create(&payer).await?;
        Ok(payer)
    }

    pub async fn get_payer_by_id(&self, payer_id: &str) -> Result<Option<InsPayer>> {
        self.repo.get_by_id(payer_id).await
    }

    pub async fn get_payer_by_code(&self, code: &str) -> Result<Option<InsPayer>> {
        self.repo.get_by_code(code).await
    }

    pub async fn list_payers(&self, limit: i64, offset: i64) -> Result<Vec<InsPayer>> {
        self.repo.list(limit, offset).await
    }

    pub async fn count_payers(&self) -> Result<i64> {
        self.repo.count().await
    }

    pub async fn update_payer(&self, payer: &InsPayer) -> Result<()> {
        self.repo.update(payer).await
    }

    pub async fn delete_payer(&self, payer_id: &str) -> Result<()> {
        self.repo.delete(payer_id).await
    }
}
