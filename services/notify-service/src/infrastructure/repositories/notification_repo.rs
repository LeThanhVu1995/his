use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::entities::notification::{
    Notification, NotificationTarget, NotificationWithTarget, NotificationStats
};
use anyhow::Result;

pub struct NotificationRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> NotificationRepo<'a> {
    pub async fn create(&self, notification: &Notification) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO notification (notification_id, code, title, body, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(notification.notification_id)
        .bind(&notification.code)
        .bind(&notification.title)
        .bind(&notification.body)
        .bind(notification.created_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, notification_id: Uuid) -> Result<Option<Notification>> {
        Ok(sqlx::query_as::<_, Notification>(
            r#"
            SELECT notification_id, code, title, body, created_at
            FROM notification
            WHERE notification_id = $1
            "#
        )
        .bind(notification_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(
        &self,
        code: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>> {
        let mut query = r#"
            SELECT notification_id, code, title, body, created_at
            FROM notification
            WHERE 1 = 1
        "#.to_string();

        if let Some(c) = code {
            query.push_str(&format!(" AND code = '{}'", c));
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT {} OFFSET {}", limit, offset));

        let notifications = sqlx::query_as::<_, Notification>(&query)
            .fetch_all(self.db)
            .await?;
        Ok(notifications)
    }

    pub async fn update(&self, notification_id: Uuid, notification: &Notification) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE notification
            SET code = $1, title = $2, body = $3
            WHERE notification_id = $4
            "#
        )
        .bind(&notification.code)
        .bind(&notification.title)
        .bind(&notification.body)
        .bind(notification_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, notification_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM notification WHERE notification_id = $1")
            .bind(notification_id)
            .execute(self.db)
            .await?;
        Ok(())
    }
}

pub struct NotificationTargetRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> NotificationTargetRepo<'a> {
    pub async fn create(&self, target: &NotificationTarget) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO notification_target (notification_id, user_id, read_at)
            VALUES ($1, $2, $3)
            "#
        )
        .bind(target.notification_id)
        .bind(target.user_id)
        .bind(target.read_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
        unread_only: Option<bool>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<NotificationWithTarget>> {
        let mut query = r#"
            SELECT n.notification_id, n.code, n.title, n.body, n.created_at, nt.read_at,
                   CASE WHEN nt.read_at IS NULL THEN false ELSE true END as is_read
            FROM notification n
            INNER JOIN notification_target nt ON n.notification_id = nt.notification_id
            WHERE nt.user_id = $1
        "#.to_string();

        if let Some(true) = unread_only {
            query.push_str(" AND nt.read_at IS NULL");
        }

        query.push_str(&format!(" ORDER BY n.created_at DESC LIMIT {} OFFSET {}", limit, offset));

        let notifications = sqlx::query_as::<_, NotificationWithTarget>(&query)
            .bind(user_id)
            .fetch_all(self.db)
            .await?;
        Ok(notifications)
    }

    pub async fn mark_as_read(&self, notification_id: Uuid, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE notification_target
            SET read_at = CURRENT_TIMESTAMP
            WHERE notification_id = $1 AND user_id = $2
            "#
        )
        .bind(notification_id)
        .bind(user_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_user_stats(&self, user_id: Uuid) -> Result<NotificationStats> {
        let stats = sqlx::query_as::<_, NotificationStats>(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(CASE WHEN read_at IS NULL THEN 1 END) as unread,
                COUNT(CASE WHEN read_at IS NOT NULL THEN 1 END) as read
            FROM notification_target
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_one(self.db)
        .await?;
        Ok(stats)
    }

    pub async fn assign_to_users(&self, notification_id: Uuid, user_ids: Vec<Uuid>) -> Result<()> {
        let mut tx = self.db.begin().await?;

        for user_id in user_ids {
            sqlx::query(
                r#"
                INSERT INTO notification_target (notification_id, user_id, read_at)
                VALUES ($1, $2, NULL)
                ON CONFLICT (notification_id, user_id) DO NOTHING
                "#
            )
            .bind(notification_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
