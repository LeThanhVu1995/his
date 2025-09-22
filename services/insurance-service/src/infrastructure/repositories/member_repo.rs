use sqlx::{Pool, Postgres};
use crate::domain::entities::member::Member;

pub struct MemberRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> MemberRepo<'a> {
    pub async fn find_by_policy(&self, payer: &str, policy: &str) -> anyhow::Result<Option<Member>> {
        Ok(sqlx::query_as::<_, Member>(
            r#"SELECT id,patient_id,payer,policy_no,plan_code,start_date,end_date,status,holder_name,note,created_at,updated_at FROM ins_members WHERE payer=$1 AND policy_no=$2"#
        )
        .bind(payer)
        .bind(policy)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn upsert(&self, m: &Member) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO ins_members(id,patient_id,payer,policy_no,plan_code,start_date,end_date,status,holder_name,note) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON CONFLICT(payer,policy_no) DO UPDATE SET patient_id=EXCLUDED.patient_id, plan_code=EXCLUDED.plan_code, start_date=EXCLUDED.start_date, end_date=EXCLUDED.end_date, status=EXCLUDED.status, holder_name=EXCLUDED.holder_name, note=EXCLUDED.note, updated_at=NOW()"#
        )
        .bind(m.id)
        .bind(m.patient_id)
        .bind(&m.payer)
        .bind(&m.policy_no)
        .bind(&m.plan_code)
        .bind(m.start_date)
        .bind(m.end_date)
        .bind(&m.status)
        .bind(&m.holder_name)
        .bind(&m.note)
        .execute(self.db)
        .await?;
        Ok(())
    }
}
