use uuid::Uuid;
use sqlx::{Pool, Postgres};
use serde_json::Value;
use crate::infra::db::repositories::device_repo::DeviceRepo;

pub struct IngestSvc<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> IngestSvc<'a> {
    pub async fn upsert_device(&self, code: &str, name: &str, r#type: &str, location: Option<&str>) -> anyhow::Result<Uuid> {
        let device = DeviceRepo { db: self.db }
            .upsert(code, name, r#type, location)
            .await?;
        Ok(device.id)
    }

    pub async fn ingest_vital_json(&self, device_code: &str, json: &Value) -> anyhow::Result<Uuid> {
        // Get or create device
        let device = DeviceRepo { db: self.db }
            .get_by_code(device_code)
            .await?;

        let device_id = if let Some(device) = device {
            device.id
        } else {
            // Auto-create device if not exists
            let new_device = DeviceRepo { db: self.db }
                .upsert(device_code, &format!("Auto-created device: {}", device_code), "SENSOR", None)
                .await?;
            new_device.id
        };

        // Parse vital data from JSON
        if let Some(vitals) = json.get("vitals").and_then(|v| v.as_array()) {
            for vital in vitals {
                if let (Some(code), Some(value)) = (vital.get("code").and_then(|c| c.as_str()), vital.get("value")) {
                    // Create vital sign record
                    let vs_id = Uuid::new_v4();
                    let measured_at = chrono::Utc::now();

                    // Insert vital sign record
                    sqlx::query(
                        r#"
                        INSERT INTO vital_sign_record (vs_id, encounter_id, patient_id, device_id, measured_at, recorder_staff_id, note)
                        VALUES ($1, $2, $3, $4, $5, $6, $7)
                        "#
                    )
                    .bind(&vs_id)
                    .bind(Uuid::new_v4()) // Placeholder encounter_id
                    .bind(Uuid::new_v4()) // Placeholder patient_id
                    .bind(&device_id)
                    .bind(&measured_at)
                    .bind(None::<Uuid>)
                    .bind(Some("Auto-ingested from IoT device"))
                    .execute(self.db)
                    .await?;

                    // Insert vital sign item
                    let vs_item_id = Uuid::new_v4();
                    let value_num = value.as_f64().map(|v| rust_decimal::Decimal::from_f64_retain(v));
                    let unit = vital.get("unit").and_then(|u| u.as_str());

                    sqlx::query(
                        r#"
                        INSERT INTO vital_sign_item (vs_item_id, vs_id, code, value_num, value_text, unit)
                        VALUES ($1, $2, $3, $4, $5, $6)
                        "#
                    )
                    .bind(&vs_item_id)
                    .bind(&vs_id)
                    .bind(code)
                    .bind(value_num)
                    .bind(value.as_str())
                    .bind(unit)
                    .execute(self.db)
                    .await?;
                }
            }
        }

        // Update device last_seen
        DeviceRepo { db: self.db }
            .touch_seen(device_id)
            .await?;

        Ok(device_id)
    }
}
