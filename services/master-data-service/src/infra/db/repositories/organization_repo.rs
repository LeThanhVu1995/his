use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::{
    OrgHospital, OrgFacility, OrgDepartment, OrgRoom, OrgBed
};
use anyhow::Result;

pub struct OrgHospitalRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> OrgHospitalRepo<'a> {
    pub async fn create(&self, hospital: &OrgHospital) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO org_hospital (hospital_id, code, name, status, created_at, created_by, updated_at, updated_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(hospital.hospital_id)
        .bind(&hospital.code)
        .bind(&hospital.name)
        .bind(&hospital.status)
        .bind(hospital.created_at)
        .bind(hospital.created_by)
        .bind(hospital.updated_at)
        .bind(hospital.updated_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, hospital_id: Uuid) -> Result<Option<OrgHospital>> {
        Ok(sqlx::query_as::<_, OrgHospital>(
            r#"
            SELECT hospital_id, code, name, status, created_at, created_by, updated_at, updated_by, deleted_at, deleted_by
            FROM org_hospital
            WHERE hospital_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(hospital_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn get_by_code(&self, code: &str) -> Result<Option<OrgHospital>> {
        Ok(sqlx::query_as::<_, OrgHospital>(
            r#"
            SELECT hospital_id, code, name, status, created_at, created_by, updated_at, updated_by, deleted_at, deleted_by
            FROM org_hospital
            WHERE code = $1 AND deleted_at IS NULL
            "#
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(
        &self,
        code: Option<String>,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<OrgHospital>> {
        let mut query = r#"
            SELECT hospital_id, code, name, status, created_at, created_by, updated_at, updated_by, deleted_at, deleted_by
            FROM org_hospital
            WHERE deleted_at IS NULL
        "#.to_string();

        if let Some(c) = code {
            query.push_str(&format!(" AND code ILIKE '%{}%'", c));
        }
        if let Some(s) = status {
            query.push_str(&format!(" AND status = '{}'", s));
        }

        query.push_str(&format!(" ORDER BY code LIMIT {} OFFSET {}", limit, offset));

        let hospitals = sqlx::query_as::<_, OrgHospital>(&query)
            .fetch_all(self.db)
            .await?;
        Ok(hospitals)
    }

    pub async fn update(&self, hospital_id: Uuid, hospital: &OrgHospital) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE org_hospital
            SET name = $1, status = $2, updated_at = $3, updated_by = $4
            WHERE hospital_id = $5 AND deleted_at IS NULL
            "#
        )
        .bind(&hospital.name)
        .bind(&hospital.status)
        .bind(hospital.updated_at)
        .bind(hospital.updated_by)
        .bind(hospital_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn soft_delete(&self, hospital_id: Uuid, deleted_by: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE org_hospital
            SET deleted_at = CURRENT_TIMESTAMP, deleted_by = $1
            WHERE hospital_id = $2 AND deleted_at IS NULL
            "#
        )
        .bind(deleted_by)
        .bind(hospital_id)
        .execute(self.db)
        .await?;
        Ok(())
    }
}

pub struct OrgFacilityRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> OrgFacilityRepo<'a> {
    pub async fn create(&self, facility: &OrgFacility) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO org_facility (facility_id, hospital_id, code, name, address_line1, address_line2,
                                    district, city, province, country, postal_code, status,
                                    created_at, created_by, updated_at, updated_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#
        )
        .bind(facility.facility_id)
        .bind(facility.hospital_id)
        .bind(&facility.code)
        .bind(&facility.name)
        .bind(&facility.address_line1)
        .bind(&facility.address_line2)
        .bind(&facility.district)
        .bind(&facility.city)
        .bind(&facility.province)
        .bind(&facility.country)
        .bind(&facility.postal_code)
        .bind(&facility.status)
        .bind(facility.created_at)
        .bind(facility.created_by)
        .bind(facility.updated_at)
        .bind(facility.updated_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, facility_id: Uuid) -> Result<Option<OrgFacility>> {
        Ok(sqlx::query_as::<_, OrgFacility>(
            r#"
            SELECT facility_id, hospital_id, code, name, address_line1, address_line2,
                   district, city, province, country, postal_code, status,
                   created_at, created_by, updated_at, updated_by, deleted_at, deleted_by
            FROM org_facility
            WHERE facility_id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(facility_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(
        &self,
        hospital_id: Option<Uuid>,
        code: Option<String>,
        status: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<OrgFacility>> {
        let mut query = r#"
            SELECT facility_id, hospital_id, code, name, address_line1, address_line2,
                   district, city, province, country, postal_code, status,
                   created_at, created_by, updated_at, updated_by, deleted_at, deleted_by
            FROM org_facility
            WHERE deleted_at IS NULL
        "#.to_string();

        if let Some(h_id) = hospital_id {
            query.push_str(&format!(" AND hospital_id = '{}'", h_id));
        }
        if let Some(c) = code {
            query.push_str(&format!(" AND code ILIKE '%{}%'", c));
        }
        if let Some(s) = status {
            query.push_str(&format!(" AND status = '{}'", s));
        }

        query.push_str(&format!(" ORDER BY code LIMIT {} OFFSET {}", limit, offset));

        let facilities = sqlx::query_as::<_, OrgFacility>(&query)
            .fetch_all(self.db)
            .await?;
        Ok(facilities)
    }
}
