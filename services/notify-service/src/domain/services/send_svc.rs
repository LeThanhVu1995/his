use crate::infrastructure::channels::{self, Channel};
use crate::infrastructure::repositories::message_repo::MessageRepo;
use crate::domain::entities::message::Message;
use uuid::Uuid;

pub struct SendSvc<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> SendSvc<'a> {
    pub async fn send_now(&self, m: &Message) -> anyhow::Result<()> {
        let repo = MessageRepo { db: self.db };

        match Channel::from(m.channel.as_str()) {
            Channel::EMAIL => {
                channels::email::send(&m.to_addr, m.subject.as_deref().unwrap_or("(no subject)"), &m.body).await?;
                repo.mark_sent(m.id).await?;
            },
            Channel::SMS => {
                channels::sms::send(&m.to_addr, &m.body).await?;
                repo.mark_sent(m.id).await?;
            },
            Channel::PUSH => {
                channels::push::send(&m.to_addr, m.subject.as_deref().unwrap_or(""), &m.body).await?;
                repo.mark_sent(m.id).await?;
            },
            Channel::INAPP => {
                channels::inapp::store(self.db, &m.to_addr, m.subject.as_deref().unwrap_or(""), &m.body).await?;
                repo.mark_sent(m.id).await?;
            },
        };
        Ok(())
    }

    pub async fn enqueue_and_send(&self, m: Message) -> anyhow::Result<Uuid> {
        let repo = MessageRepo { db: self.db };
        repo.enqueue(&m).await?;
        self.send_now(&m).await?;
        Ok(m.id)
    }
}
