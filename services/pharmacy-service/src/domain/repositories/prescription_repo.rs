use anyhow::Result;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::prescription::{
    Prescription, PrescriptionItem, CreatePrescriptionRequest, UpdatePrescriptionRequest,
    CreatePrescriptionItemRequest, UpdatePrescriptionItemRequest, PrescriptionWithItems, PrescriptionStats
};

pub struct PrescriptionRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> PrescriptionRepo<'a> {
    pub async fn create(&self, prescription: &CreatePrescriptionRequest) -> Result<Prescription> {
        let new_prescription = sqlx::query_as::<_, Prescription>(
            r#"
            INSERT INTO prescription (prescription_id, encounter_id, patient_id, prescriber_id, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, COALESCE($5, 'ACTIVE'), $6, $7)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(prescription.encounter_id)
        .bind(prescription.patient_id)
        .bind(prescription.prescriber_id)
        .bind(&prescription.status)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(self.db)
        .await?;

        Ok(new_prescription)
    }

    pub async fn create_item(&self, prescription_id: Uuid, item: &CreatePrescriptionItemRequest) -> Result<PrescriptionItem> {
        let new_item = sqlx::query_as::<_, PrescriptionItem>(
            r#"
            INSERT INTO prescription_item (
                prescription_item_id, prescription_id, drug_id, dose_per_take, dose_unit,
                frequency_text, route_code, duration_days, quantity, quantity_unit, instructions,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(prescription_id)
        .bind(item.drug_id)
        .bind(item.dose_per_take)
        .bind(&item.dose_unit)
        .bind(&item.frequency_text)
        .bind(&item.route_code)
        .bind(item.duration_days)
        .bind(item.quantity)
        .bind(&item.quantity_unit)
        .bind(&item.instructions)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(self.db)
        .await?;

        Ok(new_item)
    }

    pub async fn get_by_id(&self, prescription_id: Uuid) -> Result<Option<Prescription>> {
        let prescription = sqlx::query_as::<_, Prescription>(
            r#"
            SELECT * FROM prescription WHERE prescription_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(prescription_id)
        .fetch_optional(self.db)
        .await?;

        Ok(prescription)
    }

    pub async fn get_with_items(&self, prescription_id: Uuid) -> Result<Option<PrescriptionWithItems>> {
        let prescription = self.get_by_id(prescription_id).await?;
        if let Some(prescription) = prescription {
            let items = self.get_items_by_prescription_id(prescription_id).await?;
            Ok(Some(PrescriptionWithItems {
                prescription,
                items,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_items_by_prescription_id(&self, prescription_id: Uuid) -> Result<Vec<PrescriptionItem>> {
        let items = sqlx::query_as::<_, PrescriptionItem>(
            r#"
            SELECT * FROM prescription_item WHERE prescription_id = $1 AND deleted_at IS NULL
            ORDER BY created_at ASC
            "#
        )
        .bind(prescription_id)
                .fetch_all(self.db)
                .await?;
        Ok(items)
    }

    pub async fn list_paged(
        &self,
        patient_id: Option<Uuid>,
        encounter_id: Option<Uuid>,
        prescriber_id: Option<Uuid>,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Prescription>> {
        let mut query = r#"
            SELECT * FROM prescription WHERE deleted_at IS NULL
        "#.to_string();
        let mut args = Vec::new();
        let mut arg_idx = 1;

        if let Some(p_id) = patient_id {
            query.push_str(&format!(" AND patient_id = ${}", arg_idx));
            args.push(sqlx::query_scalar("SELECT $1::uuid").bind(p_id).fetch_one(self.db).await?);
            arg_idx += 1;
        }
        if let Some(e_id) = encounter_id {
            query.push_str(&format!(" AND encounter_id = ${}", arg_idx));
            args.push(sqlx::query_scalar("SELECT $1::uuid").bind(e_id).fetch_one(self.db).await?);
            arg_idx += 1;
        }
        if let Some(pr_id) = prescriber_id {
            query.push_str(&format!(" AND prescriber_id = ${}", arg_idx));
            args.push(sqlx::query_scalar("SELECT $1::uuid").bind(pr_id).fetch_one(self.db).await?);
            arg_idx += 1;
        }
        if let Some(s) = status {
            query.push_str(&format!(" AND status = ${}", arg_idx));
            args.push(sqlx::query_scalar("SELECT $1::text").bind(s).fetch_one(self.db).await?);
            arg_idx += 1;
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", arg_idx, arg_idx + 1));

        let prescriptions = sqlx::query_as::<_, Prescription>(&query)
            .bind(limit)
                .bind(offset)
                .fetch_all(self.db)
                .await?;
        Ok(prescriptions)
    }

    pub async fn update(&self, prescription_id: Uuid, prescription: &UpdatePrescriptionRequest) -> Result<Prescription> {
        let updated_prescription = sqlx::query_as::<_, Prescription>(
            r#"
            UPDATE prescription
            SET status = COALESCE($2, status),
                prescriber_id = COALESCE($3, prescriber_id),
                updated_at = $4
            WHERE prescription_id = $1 AND deleted_at IS NULL
            RETURNING *
            "#
        )
        .bind(prescription_id)
        .bind(&prescription.status)
        .bind(prescription.prescriber_id)
        .bind(Utc::now())
                    .fetch_one(self.db)
                    .await?;

        Ok(updated_prescription)
    }

    pub async fn update_item(&self, prescription_item_id: Uuid, item: &UpdatePrescriptionItemRequest) -> Result<PrescriptionItem> {
        let updated_item = sqlx::query_as::<_, PrescriptionItem>(
            r#"
            UPDATE prescription_item
            SET dose_per_take = COALESCE($2, dose_per_take),
                dose_unit = COALESCE($3, dose_unit),
                frequency_text = COALESCE($4, frequency_text),
                route_code = COALESCE($5, route_code),
                duration_days = COALESCE($6, duration_days),
                quantity = COALESCE($7, quantity),
                quantity_unit = COALESCE($8, quantity_unit),
                instructions = COALESCE($9, instructions),
                updated_at = $10
            WHERE prescription_item_id = $1 AND deleted_at IS NULL
            RETURNING *
            "#
        )
        .bind(prescription_item_id)
        .bind(item.dose_per_take)
        .bind(&item.dose_unit)
        .bind(&item.frequency_text)
        .bind(&item.route_code)
        .bind(item.duration_days)
        .bind(item.quantity)
        .bind(&item.quantity_unit)
        .bind(&item.instructions)
        .bind(Utc::now())
                    .fetch_one(self.db)
                    .await?;

        Ok(updated_item)
    }

    pub async fn delete(&self, prescription_id: Uuid, deleted_by: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE prescription
            SET deleted_at = $2, deleted_by = $3
            WHERE prescription_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(prescription_id)
        .bind(Utc::now())
        .bind(deleted_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_item(&self, prescription_item_id: Uuid, deleted_by: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE prescription_item
            SET deleted_at = $2, deleted_by = $3
            WHERE prescription_item_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(prescription_item_id)
        .bind(Utc::now())
        .bind(deleted_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_stats(&self) -> Result<PrescriptionStats> {
        // For now, return placeholder stats
        // In a real implementation, you'd use raw query and map to struct
        Ok(PrescriptionStats {
            total: 0,
            active: 0,
            completed: 0,
            cancelled: 0,
        })
    }

    // Placeholder methods for compatibility with existing handlers
    pub async fn find(&self, prescription_id: Uuid) -> Result<Option<Prescription>> {
        self.get_by_id(prescription_id).await
    }
}