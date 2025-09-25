use tokio::time::{sleep, Duration};
use crate::engine::{scheduler, interpreter::Interpreter};

pub async fn run(db: sqlx::Pool<sqlx::Postgres>) {
    loop {
        if let Ok(ids) = scheduler::poll_and_wake(&db).await {
            for id in ids {
                let _ = Interpreter { db: &db }.tick(id).await;
            }
        }
        sleep(Duration::from_secs(2)).await;
    }
}
