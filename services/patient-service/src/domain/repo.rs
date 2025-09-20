use crate::domain::models::{Patient, Encounter};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct PatientRepo<'a> {
    pub db: &'a Pool<Postgres>
}

impl<'a> PatientRepo<'a> {
    pub async fn create(&self, p: &Patient) -> anyhow::Result<()> {
        sqlx::query(r#"
            INSERT INTO patients (id, mrn, national_id, passport_no, full_name, first_name, last_name, gender, birth_date, phone, email, address, blood_type, marital_status, is_active)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
        "#)
        .bind(p.id)
        .bind(&p.mrn)
        .bind(&p.national_id)
        .bind(&p.passport_no)
        .bind(&p.full_name)
        .bind(&p.first_name)
        .bind(&p.last_name)
        .bind(&p.gender)
        .bind(p.birth_date)
        .bind(&p.phone)
        .bind(&p.email)
        .bind(&p.address)
        .bind(&p.blood_type)
        .bind(&p.marital_status)
        .bind(p.is_active)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, full_name: Option<&str>, gender: Option<&str>, birth_date: Option<chrono::NaiveDate>, phone: Option<&str>, email: Option<&str>, address: Option<&str>, is_active: Option<bool>) -> anyhow::Result<Option<Patient>> {
        let rec = sqlx::query_as::<_, Patient>(r#"
            UPDATE patients
            SET full_name = COALESCE($2, full_name),
                gender = COALESCE($3, gender),
                birth_date = COALESCE($4, birth_date),
                phone = COALESCE($5, phone),
                email = COALESCE($6, email),
                address = COALESCE($7, address),
                is_active = COALESCE($8, is_active),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, mrn, national_id, passport_no, full_name, first_name, last_name, gender, birth_date, phone, email, address, blood_type, marital_status, is_active, created_at, updated_at
        "#)
        .bind(id)
        .bind(full_name)
        .bind(gender)
        .bind(birth_date)
        .bind(phone)
        .bind(email)
        .bind(address)
        .bind(is_active)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Patient>> {
        let rec = sqlx::query_as::<_, Patient>(r#"
            SELECT id, mrn, national_id, passport_no, full_name, first_name, last_name, gender, birth_date, phone, email, address, blood_type, marital_status, is_active, created_at, updated_at
            FROM patients WHERE id = $1
        "#)
        .bind(id)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn list_paged(&self, keyword: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Patient>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page-1)*size;

        if let Some(q) = keyword { // search by name or MRN
            let like = format!("%{}%", q);
            let rows = sqlx::query_as::<_, Patient>(r#"
                SELECT id, mrn, national_id, passport_no, full_name, first_name, last_name, gender, birth_date, phone, email, address, blood_type, marital_status, is_active, created_at, updated_at
                FROM patients
                WHERE full_name ILIKE $1 OR mrn ILIKE $1
                ORDER BY full_name OFFSET $2 LIMIT $3
            "#)
            .bind(&like)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let total: i64 = sqlx::query_scalar(r#"
                SELECT COUNT(1) FROM patients WHERE full_name ILIKE $1 OR mrn ILIKE $1
            "#)
            .bind(&like)
            .fetch_one(self.db)
            .await?;
            Ok((rows, total))
        } else {
            let rows = sqlx::query_as::<_, Patient>(r#"
                SELECT id, mrn, national_id, passport_no, full_name, first_name, last_name, gender, birth_date, phone, email, address, blood_type, marital_status, is_active, created_at, updated_at
                FROM patients
                ORDER BY full_name OFFSET $1 LIMIT $2
            "#)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let total: i64 = sqlx::query_scalar(r#"
                SELECT COUNT(1) FROM patients
            "#)
            .fetch_one(self.db)
            .await?;
            Ok((rows, total))
        }
    }
}

pub struct EncounterRepo<'a> {
    pub db: &'a Pool<Postgres>
}

impl<'a> EncounterRepo<'a> {
    pub async fn create(&self, e: &Encounter) -> anyhow::Result<()> {
        sqlx::query(r#"
            INSERT INTO encounters (id, patient_id, encounter_no, encounter_type, status, department_code, attending_doctor_id, admitted_at, discharged_at)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
        "#)
        .bind(e.id)
        .bind(e.patient_id)
        .bind(&e.encounter_no)
        .bind(&e.encounter_type)
        .bind(&e.status)
        .bind(&e.department_code)
        .bind(&e.attending_doctor_id)
        .bind(e.admitted_at)
        .bind(e.discharged_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, status: Option<&str>, dept: Option<&str>, doctor: Option<&str>) -> anyhow::Result<Option<Encounter>> {
        let rec = sqlx::query_as::<_, Encounter>(r#"
            UPDATE encounters
            SET status = COALESCE($2, status),
                department_code = COALESCE($3, department_code),
                attending_doctor_id = COALESCE($4, attending_doctor_id),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, patient_id, encounter_no, encounter_type, status, department_code, attending_doctor_id, admitted_at, discharged_at, created_at, updated_at
        "#)
        .bind(id)
        .bind(status)
        .bind(dept)
        .bind(doctor)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn close(&self, id: Uuid) -> anyhow::Result<Option<Encounter>> {
        let rec = sqlx::query_as::<_, Encounter>(r#"
            UPDATE encounters
            SET status = 'FINISHED', discharged_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND status <> 'FINISHED'
            RETURNING id, patient_id, encounter_no, encounter_type, status, department_code, attending_doctor_id, admitted_at, discharged_at, created_at, updated_at
        "#)
        .bind(id)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Encounter>> {
        let rec = sqlx::query_as::<_, Encounter>(r#"
            SELECT id, patient_id, encounter_no, encounter_type, status, department_code, attending_doctor_id, admitted_at, discharged_at, created_at, updated_at
            FROM encounters WHERE id = $1
        "#)
        .bind(id)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn list_paged(&self, patient_id: Option<Uuid>, status: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Encounter>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page-1)*size;

        match (patient_id, status) {
            (Some(pid), Some(st)) => {
                let rows = sqlx::query_as::<_, Encounter>(r#"
                    SELECT id, patient_id, encounter_no, encounter_type, status, department_code, attending_doctor_id, admitted_at, discharged_at, created_at, updated_at
                    FROM encounters WHERE patient_id=$1 AND status=$2
                    ORDER BY created_at DESC OFFSET $3 LIMIT $4
                "#)
                .bind(pid)
                .bind(st)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM encounters WHERE patient_id=$1 AND status=$2
                "#)
                .bind(pid)
                .bind(st)
                .fetch_one(self.db)
                .await?;
                Ok((rows, total))
            }
            (Some(pid), None) => {
                let rows = sqlx::query_as::<_, Encounter>(r#"
                    SELECT id, patient_id, encounter_no, encounter_type, status, department_code, attending_doctor_id, admitted_at, discharged_at, created_at, updated_at
                    FROM encounters WHERE patient_id=$1
                    ORDER BY created_at DESC OFFSET $2 LIMIT $3
                "#)
                .bind(pid)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM encounters WHERE patient_id=$1
                "#)
                .bind(pid)
                .fetch_one(self.db)
                .await?;
                Ok((rows, total))
            }
            (None, Some(st)) => {
                let rows = sqlx::query_as::<_, Encounter>(r#"
                    SELECT id, patient_id, encounter_no, encounter_type, status, department_code, attending_doctor_id, admitted_at, discharged_at, created_at, updated_at
                    FROM encounters WHERE status=$1
                    ORDER BY created_at DESC OFFSET $2 LIMIT $3
                "#)
                .bind(st)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM encounters WHERE status=$1
                "#)
                .bind(st)
                .fetch_one(self.db)
                .await?;
                Ok((rows, total))
            }
            (None, None) => {
                let rows = sqlx::query_as::<_, Encounter>(r#"
                    SELECT id, patient_id, encounter_no, encounter_type, status, department_code, attending_doctor_id, admitted_at, discharged_at, created_at, updated_at
                    FROM encounters
                    ORDER BY created_at DESC OFFSET $1 LIMIT $2
                "#)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let total: i64 = sqlx::query_scalar(r#"
                    SELECT COUNT(1) FROM encounters
                "#)
                .fetch_one(self.db)
                .await?;
                Ok((rows, total))
            }
        }
    }
}
