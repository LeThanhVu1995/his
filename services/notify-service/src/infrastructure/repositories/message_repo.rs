use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::message::Message;

pub struct MessageRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> MessageRepo<'a> {
    pub async fn enqueue(&self, m: &Message) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO notify_messages(id,template_code,channel,to_addr,subject,body,status,err) VALUES($1,$2,$3,$4,$5,$6,$7,$8)"#
        )
        .bind(m.id)
        .bind(&m.template_code)
        .bind(&m.channel)
        .bind(&m.to_addr)
        .bind(&m.subject)
        .bind(&m.body)
        .bind(&m.status)
        .bind(&m.err)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn mark_sent(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE notify_messages SET status='SENT', sent_at=NOW(), err=NULL WHERE id=$1"#
        )
        .bind(id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn mark_failed(&self, id: Uuid, err: &str) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE notify_messages SET status='FAILED', err=$2 WHERE id=$1"#
        )
        .bind(id)
        .bind(err)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, id: Uuid) -> anyhow::Result<Option<Message>> {
        Ok(sqlx::query_as::<_, Message>(
            r#"SELECT id,template_code,channel,to_addr,subject,body,status,err,created_at,sent_at FROM notify_messages WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list(&self, channel: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Message>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = if let Some(ch) = channel {
            let r = sqlx::query_as::<_, Message>(
                r#"SELECT id,template_code,channel,to_addr,subject,body,status,err,created_at,sent_at FROM notify_messages WHERE channel=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#
            )
            .bind(ch)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM notify_messages WHERE channel=$1"#
            )
            .bind(ch)
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, Message>(
                r#"SELECT id,template_code,channel,to_addr,subject,body,status,err,created_at,sent_at FROM notify_messages ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
            )
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM notify_messages"#
            )
            .fetch_one(self.db)
            .await?;
            (r, t)
        };
        Ok((rows, total))
    }
}
