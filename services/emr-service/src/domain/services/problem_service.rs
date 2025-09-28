use anyhow::Result;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::problem::ProblemList;
use crate::infra::db::repositories::ProblemRepo;
use crate::http::dto::problem::*;

pub struct ProblemService<'a> {
    repo: ProblemRepo<'a>,
}

impl<'a> ProblemService<'a> {
    pub fn new(repo: ProblemRepo<'a>) -> Self {
        Self { repo }
    }

    // Problem List CRUD operations
    pub async fn create_problem(&self, req: CreateProblemRequest) -> Result<ProblemList> {
        let problem_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let problem = ProblemList {
            problem_id: problem_id.clone(),
            patient_id: req.patient_id,
            code: Some(req.problem_code),
            description: req.description,
            onset_date: req.onset_date,
            abatement_date: req.abatement_date,
            status: req.status.unwrap_or_else(|| "active".to_string()),
        };

        self.repo.create_problem(&problem).await?;
        Ok(problem)
    }

    pub async fn get_problem(&self, problem_id: &str) -> Result<Option<ProblemList>> {
        self.repo.get_problem(problem_id).await
    }

    pub async fn list_patient_problems(
        &self,
        patient_id: &str,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<ProblemList>, i64)> {
        let problems = self.repo.list_patient_problems(patient_id, limit, offset).await?;
        let total = self.repo.count_patient_problems(patient_id).await?;
        Ok((problems, total))
    }

    pub async fn update_problem(&self, problem_id: &str, req: UpdateProblemRequest) -> Result<ProblemList> {
        let mut problem = self.repo.get_problem(problem_id).await?
            .ok_or_else(|| anyhow::anyhow!("Problem not found"))?;

        if let Some(problem_code) = req.problem_code {
            problem.code = Some(problem_code);
        }
        if let Some(status) = req.status {
            problem.status = status;
        }
        if let Some(description) = req.description {
            problem.description = Some(description);
        }
        if let Some(onset_date) = req.onset_date {
            problem.onset_date = Some(onset_date);
        }
        if let Some(abatement_date) = req.abatement_date {
            problem.abatement_date = Some(abatement_date);
        }

        self.repo.update_problem(&problem).await?;
        Ok(problem)
    }

    pub async fn resolve_problem(&self, problem_id: &str, abatement_date: NaiveDate, user_id: &str) -> Result<ProblemList> {
        let mut problem = self.repo.get_problem(problem_id).await?
            .ok_or_else(|| anyhow::anyhow!("Problem not found"))?;

        problem.status = "resolved".to_string();
        problem.abatement_date = Some(abatement_date);

        self.repo.update_problem(&problem).await?;
        Ok(problem)
    }

    pub async fn delete_problem(&self, problem_id: &str, user_id: &str) -> Result<()> {
        self.repo.delete_problem(problem_id, user_id).await
    }
}
