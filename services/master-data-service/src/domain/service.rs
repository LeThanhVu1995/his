use crate::domain::repo::MasterRepo;
use crate::infrastructure::kafka::Kafka;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct CodeEvent<'a, T> {
    event: &'a str,
    data: T,
}

pub struct MasterService<'a> {
    pub repo: MasterRepo<'a>,
    pub kafka: Option<&'a Kafka>,
}

impl<'a> MasterService<'a> {
    pub async fn create_code(&self, category: &str, code: &str, name: &str, desc: Option<&str>) -> anyhow::Result<uuid::Uuid> {
        let created = self.repo.create_code(category, code, name, desc).await?;
        if let Some(k) = self.kafka {
            let evt = CodeEvent { event: "his.master.code.created", data: &created };
            let key = created.id.to_string();
            k.publish("his.master.code.v1", &key, &evt).await.ok();
        }
        Ok(created.id)
    }

    pub async fn update_code(&self, id: Uuid, name: Option<&str>, desc: Option<&str>, active: Option<bool>) -> anyhow::Result<bool> {
        let updated = self.repo.update_code(id, name, desc, active).await?;
        if let Some(updated) = &updated {
            if let Some(k) = self.kafka {
                let evt = CodeEvent { event: "his.master.code.updated", data: updated };
                let key = updated.id.to_string();
                k.publish("his.master.code.v1", &key, &evt).await.ok();
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
