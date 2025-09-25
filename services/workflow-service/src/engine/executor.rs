use uuid::Uuid;
use crate::engine::interpreter::Interpreter;

pub struct Executor<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> Executor<'a> {
    pub async fn execute(&self, instance_id: Uuid) -> anyhow::Result<()> {
        let interpreter = Interpreter { db: self.db };
        interpreter.tick(instance_id).await
    }
}
