use anyhow::Result;
use chrono::NaiveDate;
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::problem::ProblemList;

pub struct ProblemRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ProblemRepo<'a> {
    // Problem List CRUD operations
    pub async fn create_problem(&self, problem: &ProblemList) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO problem_list (
                problem_id, patient_id, code, description, status, onset_date, abatement_date
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(&problem.problem_id)
        .bind(&problem.patient_id)
        .bind(&problem.code)
        .bind(&problem.description)
        .bind(&problem.status)
        .bind(&problem.onset_date)
        .bind(&problem.abatement_date)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_problem(&self, problem_id: &str) -> Result<Option<ProblemList>> {
        let problem = sqlx::query_as::<_, ProblemList>(
            r#"
            SELECT problem_id, patient_id, code, description, status, onset_date, abatement_date
            FROM problem_list
            WHERE problem_id = $1
            "#
        )
        .bind(problem_id)
        .fetch_optional(self.db)
        .await?;
        Ok(problem)
    }

    pub async fn list_patient_problems(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<ProblemList>> {
        let problems = sqlx::query_as::<_, ProblemList>(
            r#"
            SELECT problem_id, patient_id, code, description, status, onset_date, abatement_date
            FROM problem_list
            WHERE patient_id = $1
            ORDER BY onset_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(problems)
    }

    pub async fn list_problems_by_status(&self, status: &str, limit: i64, offset: i64) -> Result<Vec<ProblemList>> {
        let problems = sqlx::query_as::<_, ProblemList>(
            r#"
            SELECT problem_id, patient_id, code, description, status, onset_date, abatement_date
            FROM problem_list
            WHERE status = $1
            ORDER BY onset_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(problems)
    }

    pub async fn list_problems_by_code(&self, problem_code: &str, limit: i64, offset: i64) -> Result<Vec<ProblemList>> {
        let problems = sqlx::query_as::<_, ProblemList>(
            r#"
            SELECT problem_id, patient_id, code, description, status, onset_date, abatement_date
            FROM problem_list
            WHERE code = $1
            ORDER BY onset_date DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(problem_code)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(problems)
    }

    pub async fn update_problem(&self, problem: &ProblemList) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE problem_list SET
                code = $2, description = $3, status = $4,
                onset_date = $5, abatement_date = $6
            WHERE problem_id = $1
            "#
        )
        .bind(&problem.problem_id)
        .bind(&problem.code)
        .bind(&problem.description)
        .bind(&problem.status)
        .bind(&problem.onset_date)
        .bind(&problem.abatement_date)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn resolve_problem(&self, problem_id: &str, abatement_date: NaiveDate, user_id: &str) -> Result<ProblemList> {
        sqlx::query(
            r#"
            UPDATE problem_list SET
                status = 'resolved', abatement_date = $2
            WHERE problem_id = $1
            "#
        )
        .bind(problem_id)
        .bind(abatement_date)
        .execute(self.db)
        .await?;

        // Return updated problem
        self.get_problem(problem_id).await?.ok_or_else(|| anyhow::anyhow!("Problem not found"))
    }

    pub async fn count_patient_problems(&self, patient_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM problem_list
            WHERE patient_id = $1 AND status != 'deleted'
            "#
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_problems_by_status(&self, status: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM problem_list
            WHERE status = $1
            "#
        )
        .bind(status)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_problems_by_code(&self, problem_code: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM problem_list
            WHERE code = $1
            "#
        )
        .bind(problem_code)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn delete_problem(&self, problem_id: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE problem_list SET
                status = 'deleted'
            WHERE problem_id = $1
            "#
        )
        .bind(problem_id)
        .execute(self.db)
        .await?;
        Ok(())
    }
}
