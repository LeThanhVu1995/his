use crate::infra::{opensearch::client::OsClient, db::registry_repo::RegistryRepo};
use sqlx::Row;

pub struct IndexerSvc<'a> { pub db: &'a sqlx::Pool<sqlx::Postgres>, pub os: OsClient }

impl<'a> IndexerSvc<'a> {
    pub async fn ensure_index(&self, code: &str) -> anyhow::Result<String> {
        let reg = RegistryRepo { db: self.db }.get(code).await?.ok_or(anyhow::anyhow!("no registry"))?;
        let body = serde_json::json!({ "settings": reg.settings.clone().unwrap_or(serde_json::json!({})), "mappings": reg.mapping.clone() });
        let _ = self.os.create_index(&reg.index_name, &body).await; // idempotent best-effort
        Ok(reg.index_name)
    }

    pub async fn reindex_patients(&self) -> anyhow::Result<u64> {
        let idx = self.ensure_index("patients").await?;
        let rows = sqlx::query(
            r#"SELECT
                patient_id::text as id,
                code,
                full_name,
                date_of_birth as dob,
                gender,
                phone_number as phone,
                national_id as id_no,
                CONCAT(COALESCE(address_line1, ''), ' ', COALESCE(address_line2, ''), ' ', COALESCE(city, ''), ' ', COALESCE(province, '')) as address,
                email,
                status
            FROM patient
            WHERE deleted_at IS NULL"#
        ).fetch_all(self.db).await?;
        let mut ok = 0u64;
        for r in rows {
            let id: Option<String> = r.try_get("id").ok();
            let code: Option<String> = r.try_get("code").ok();
            let full_name: Option<String> = r.try_get("full_name").ok();
            let dob: Option<chrono::NaiveDate> = r.try_get("dob").ok();
            let gender: Option<String> = r.try_get("gender").ok();
            let phone: Option<String> = r.try_get("phone").ok();
            let id_no: Option<String> = r.try_get("id_no").ok();
            let address: Option<String> = r.try_get("address").ok();
            let email: Option<String> = r.try_get("email").ok();
            let status: Option<String> = r.try_get("status").ok();
            let doc = serde_json::json!({
                "id": id,
                "code": code,
                "full_name": full_name,
                "date_of_birth": dob,
                "gender": gender,
                "phone_number": phone,
                "national_id": id_no,
                "address": address,
                "email": email,
                "status": status
            });
            if self.os.upsert_doc(&idx, &id.clone().unwrap_or_default(), &doc).await.is_ok() { ok += 1; }
        }
        Ok(ok)
    }

    pub async fn reindex_encounters(&self) -> anyhow::Result<u64> {
        let idx = self.ensure_index("encounters").await?;
        let rows = sqlx::query(
            r#"SELECT
                e.encounter_id::text as id,
                e.patient_id::text as patient_id,
                e.encounter_id::text as encounter_id,
                e.type_code,
                e.status,
                e.start_time,
                e.end_time,
                d.name as department_name,
                r.name as room_name,
                CONCAT(s.title, ' ', u.full_name) as attending_staff
            FROM encounter e
            LEFT JOIN org_department d ON e.department_id = d.department_id
            LEFT JOIN org_room r ON e.room_id = r.room_id
            LEFT JOIN staff s ON e.attending_staff_id = s.staff_id
            LEFT JOIN users u ON s.user_id = u.user_id
            WHERE e.deleted_at IS NULL"#
        ).fetch_all(self.db).await?;
        let mut ok = 0u64;
        for r in rows {
            let id: Option<String> = r.try_get("id").ok();
            let patient_id: Option<String> = r.try_get("patient_id").ok();
            let encounter_id: Option<String> = r.try_get("encounter_id").ok();
            let type_code: Option<String> = r.try_get("type_code").ok();
            let status: Option<String> = r.try_get("status").ok();
            let start_time: Option<chrono::DateTime<chrono::Utc>> = r.try_get("start_time").ok();
            let end_time: Option<chrono::DateTime<chrono::Utc>> = r.try_get("end_time").ok();
            let department_name: Option<String> = r.try_get("department_name").ok();
            let room_name: Option<String> = r.try_get("room_name").ok();
            let attending_staff: Option<String> = r.try_get("attending_staff").ok();
            let doc = serde_json::json!({
                "id": id,
                "patient_id": patient_id,
                "encounter_id": encounter_id,
                "type_code": type_code,
                "status": status,
                "start_time": start_time,
                "end_time": end_time,
                "department_name": department_name,
                "room_name": room_name,
                "attending_staff": attending_staff
            });
            if self.os.upsert_doc(&idx, &id.clone().unwrap_or_default(), &doc).await.is_ok() { ok += 1; }
        }
        Ok(ok)
    }

    pub async fn reindex_orders(&self) -> anyhow::Result<u64> {
        let idx = self.ensure_index("orders").await?;
        let rows = sqlx::query(
            r#"SELECT
                o.order_id::text as id,
                o.order_id::text as order_id,
                o.patient_id::text as patient_id,
                o.encounter_id::text as encounter_id,
                o.order_type,
                o.status,
                o.priority_code,
                o.ordered_at,
                o.remarks
            FROM clinical_order o
            WHERE o.deleted_at IS NULL"#
        ).fetch_all(self.db).await?;
        let mut ok = 0u64;
        for r in rows {
            let id: Option<String> = r.try_get("id").ok();
            let order_id: Option<String> = r.try_get("order_id").ok();
            let patient_id: Option<String> = r.try_get("patient_id").ok();
            let encounter_id: Option<String> = r.try_get("encounter_id").ok();
            let order_type: Option<String> = r.try_get("order_type").ok();
            let status: Option<String> = r.try_get("status").ok();
            let priority_code: Option<String> = r.try_get("priority_code").ok();
            let ordered_at: Option<chrono::DateTime<chrono::Utc>> = r.try_get("ordered_at").ok();
            let remarks: Option<String> = r.try_get("remarks").ok();
            let doc = serde_json::json!({
                "id": id,
                "order_id": order_id,
                "patient_id": patient_id,
                "encounter_id": encounter_id,
                "order_type": order_type,
                "status": status,
                "priority_code": priority_code,
                "ordered_at": ordered_at,
                "remarks": remarks
            });
            if self.os.upsert_doc(&idx, &id.clone().unwrap_or_default(), &doc).await.is_ok() { ok += 1; }
        }
        Ok(ok)
    }

    pub async fn reindex_documents(&self) -> anyhow::Result<u64> {
        let idx = self.ensure_index("documents").await?;
        let rows = sqlx::query(
            r#"SELECT
                d.doc_id::text as id,
                d.doc_id::text as doc_id,
                d.file_name,
                d.mime_type,
                dl.entity_name,
                dl.entity_id::text as entity_id,
                d.uploaded_at,
                d.uploaded_by::text as uploaded_by,
                dl.note
            FROM doc_file d
            LEFT JOIN doc_link dl ON d.doc_id = dl.doc_id
            WHERE d.deleted_at IS NULL"#
        ).fetch_all(self.db).await?;
        let mut ok = 0u64;
        for r in rows {
            let id: Option<String> = r.try_get("id").ok();
            let doc_id: Option<String> = r.try_get("doc_id").ok();
            let file_name: Option<String> = r.try_get("file_name").ok();
            let mime_type: Option<String> = r.try_get("mime_type").ok();
            let entity_name: Option<String> = r.try_get("entity_name").ok();
            let entity_id: Option<String> = r.try_get("entity_id").ok();
            let uploaded_at: Option<chrono::DateTime<chrono::Utc>> = r.try_get("uploaded_at").ok();
            let uploaded_by: Option<String> = r.try_get("uploaded_by").ok();
            let note: Option<String> = r.try_get("note").ok();
            let doc = serde_json::json!({
                "id": id,
                "doc_id": doc_id,
                "file_name": file_name,
                "mime_type": mime_type,
                "entity_name": entity_name,
                "entity_id": entity_id,
                "uploaded_at": uploaded_at,
                "uploaded_by": uploaded_by,
                "note": note
            });
            if self.os.upsert_doc(&idx, &id.clone().unwrap_or_default(), &doc).await.is_ok() { ok += 1; }
        }
        Ok(ok)
    }
}
