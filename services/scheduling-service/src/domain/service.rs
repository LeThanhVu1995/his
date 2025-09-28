use uuid::Uuid;
use chrono::{Duration, Datelike, Timelike};
use crate::domain::models::*;
use crate::domain::repo::{ScheduleRepo, SlotRepo, ApptRepo};

pub struct ApptService<'a> {
    pub schedules: ScheduleRepo<'a>,
    pub slots: SlotRepo<'a>,
    pub appts: ApptRepo<'a>,
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> ApptService<'a> {
    pub async fn generate_slots(&self, provider_id: Uuid, from: chrono::DateTime<chrono::Utc>, to: chrono::DateTime<chrono::Utc>) -> anyhow::Result<u64> {
        let (schedules, _) = self.schedules.list_paged(Some(provider_id), None, 1, 500).await?;
        let mut created = 0u64;

        for day in 0..=((to.date_naive() - from.date_naive()).num_days() as i64) {
            let date = from.date_naive() + chrono::Days::new(day as u64);
            let weekday = match date.weekday().num_days_from_monday() {
                0 => 1, 1 => 2, 2 => 3, 3 => 4, 4 => 5, 5 => 6, _ => 7
            } as i16;

            for sch in schedules.iter().filter(|s| s.weekday == weekday && s.active) {
                let start_dt = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                    date.and_time(sch.start_time), chrono::Utc
                );
                let end_dt = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                    date.and_time(sch.end_time), chrono::Utc
                );

                let mut cursor = start_dt;
                let slot_min = sch.slot_min as i64;

                while cursor + Duration::minutes(slot_min) <= end_dt {
                    let slot = TimeSlot {
                        id: Uuid::new_v4(),
                        provider_id,
                        room_id: sch.room_id,
                        starts_at: cursor,
                        ends_at: cursor + Duration::minutes(slot_min),
                        reserved: false,
                        locked_by: None,
                    };

                    if self.slots.upsert(&slot).await.is_ok() {
                        created += 1;
                    }
                    cursor = cursor + Duration::minutes(slot_min);
                }
            }
        }
        Ok(created)
    }

    pub async fn book(&self, patient_id: Uuid, slot_id: Uuid, reason: Option<String>, by: Option<&str>) -> anyhow::Result<Uuid> {
        let mut tx = self.db.begin().await?;

        // lock & reserve slot
        let ok = self.slots.lock_free_slot(slot_id, by.unwrap_or("system"), &mut tx).await?;
        if !ok {
            anyhow::bail!("slot already reserved");
        }

        // read slot to know provider/room
        let slot: TimeSlot = sqlx::query_as::<_, TimeSlot>(
            r#"SELECT id,provider_id,room_id,starts_at,ends_at,reserved,locked_by FROM time_slots WHERE id=$1"#
        )
        .bind(slot_id)
        .fetch_one(tx.as_mut())
        .await?;

        let appt_id = Uuid::new_v4();
        let appt = Appointment {
            id: appt_id,
            appt_no: format!("APT-{}", &appt_id.to_string()[..8]),
            patient_id,
            provider_id: slot.provider_id,
            room_id: slot.room_id,
            slot_id: slot.id,
            facility_id: None,
            department_id: None,
            staff_id: None,
            start_time: slot.starts_at,
            end_time: Some(slot.ends_at),
            status: "BOOKED".into(),
            reason,
            reason_text: None,
            created_by: by.map(|s| s.to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            updated_by: None,
            deleted_at: None,
            deleted_by: None,
        };

        self.appts.create(&appt, &mut tx).await?;
        tx.commit().await?;
        Ok(appt_id)
    }

    pub async fn cancel(&self, appt_id: Uuid) -> anyhow::Result<()> {
        // optional: free slot (giữ reserved=false để cho phép re-book)
        let rec = self.appts.set_status(appt_id, "CANCELLED").await?;
        if let Some(a) = rec {
            let _ = sqlx::query(
                "UPDATE time_slots SET reserved=FALSE, locked_by=NULL WHERE id=$1"
            )
            .bind(a.slot_id)
            .execute(self.db)
            .await?;
        }
        Ok(())
    }

    pub async fn reschedule(&self, appt_id: Uuid, new_slot_id: Uuid, by: Option<&str>) -> anyhow::Result<()> {
        let mut tx = self.db.begin().await?;

        // lock new slot
        if !self.slots.lock_free_slot(new_slot_id, by.unwrap_or("system"), &mut tx).await? {
            anyhow::bail!("new slot already reserved");
        }

        let appt: Appointment = sqlx::query_as::<_, Appointment>(
            r#"SELECT id,appt_no,patient_id,provider_id,room_id,slot_id,status,reason,created_by,created_at,updated_at FROM appointments WHERE id=$1 FOR UPDATE"#
        )
        .bind(appt_id)
        .fetch_one(tx.as_mut())
        .await?;

        // free old slot
        sqlx::query(
            "UPDATE time_slots SET reserved=FALSE, locked_by=NULL WHERE id=$1"
        )
        .bind(appt.slot_id)
        .execute(tx.as_mut())
        .await?;

        // update appointment
        let slot: TimeSlot = sqlx::query_as::<_, TimeSlot>(
            r#"SELECT id,provider_id,room_id,starts_at,ends_at,reserved,locked_by FROM time_slots WHERE id=$1"#
        )
        .bind(new_slot_id)
        .fetch_one(tx.as_mut())
        .await?;

        sqlx::query(
            r#"UPDATE appointments SET provider_id=$2, room_id=$3, slot_id=$4, updated_at=NOW() WHERE id=$1"#
        )
        .bind(appt_id)
        .bind(slot.provider_id)
        .bind(slot.room_id)
        .bind(slot.id)
        .execute(tx.as_mut())
        .await?;

        tx.commit().await?;
        Ok(())
    }
}
