use sqlx::{Pool, Postgres};

pub async fn store(db: &Pool<Postgres>, to: &str, title: &str, body: &str) -> anyhow::Result<()> {
    let id = uuid::Uuid::new_v4();
    sqlx::query(
        r#"INSERT INTO notify_messages(id,template_code,channel,to_addr,subject,body,status) VALUES($1,$2,'INAPP',$3,$4,$5,'SENT')"#
    )
    .bind(id)
    .bind(Option::<String>::None)
    .bind(to)
    .bind(title)
    .bind(body)
    .execute(db)
    .await?;
    Ok(())
}
