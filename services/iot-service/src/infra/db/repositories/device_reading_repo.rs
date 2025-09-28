use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::device_reading::DeviceReading;

pub struct DeviceReadingRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> DeviceReadingRepo<'a> {
    pub async fn create(&self, reading: &DeviceReading) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO iot_device_reading (reading_id, device_id, sensor_type, value_num, value_text, unit, quality, read_at, raw_data)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(&reading.reading_id)
        .bind(&reading.device_id)
        .bind(&reading.sensor_type)
        .bind(&reading.value_num)
        .bind(&reading.value_text)
        .bind(&reading.unit)
        .bind(&reading.quality)
        .bind(&reading.read_at)
        .bind(&reading.raw_data)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, reading_id: Uuid) -> anyhow::Result<Option<DeviceReading>> {
        let reading = sqlx::query_as::<_, DeviceReading>(
            r#"
            SELECT reading_id, device_id, sensor_type, value_num, value_text, unit, quality, read_at, raw_data
            FROM iot_device_reading
            WHERE reading_id = $1
            "#
        )
        .bind(reading_id)
        .fetch_optional(self.db)
        .await?;
        Ok(reading)
    }

    pub async fn list_by_device(&self, device_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<DeviceReading>> {
        let readings = sqlx::query_as::<_, DeviceReading>(
            r#"
            SELECT reading_id, device_id, sensor_type, value_num, value_text, unit, quality, read_at, raw_data
            FROM iot_device_reading
            WHERE device_id = $1
            ORDER BY read_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(device_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(readings)
    }

    pub async fn list_by_sensor_type(&self, sensor_type: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<DeviceReading>> {
        let readings = sqlx::query_as::<_, DeviceReading>(
            r#"
            SELECT reading_id, device_id, sensor_type, value_num, value_text, unit, quality, read_at, raw_data
            FROM iot_device_reading
            WHERE sensor_type = $1
            ORDER BY read_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(sensor_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(readings)
    }

    pub async fn list_recent_by_device(&self, device_id: Uuid, hours: i64) -> anyhow::Result<Vec<DeviceReading>> {
        let readings = sqlx::query_as::<_, DeviceReading>(
            r#"
            SELECT reading_id, device_id, sensor_type, value_num, value_text, unit, quality, read_at, raw_data
            FROM iot_device_reading
            WHERE device_id = $1 AND read_at >= NOW() - INTERVAL '$2 hours'
            ORDER BY read_at DESC
            "#
        )
        .bind(device_id)
        .bind(hours)
        .fetch_all(self.db)
        .await?;
        Ok(readings)
    }

    pub async fn count_by_device(&self, device_id: Uuid) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM iot_device_reading WHERE device_id = $1"
        )
        .bind(device_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }
}
