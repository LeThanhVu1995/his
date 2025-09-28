use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::entities::charge::{Charge, PriceList, PriceItem};

#[derive(Clone)]
pub struct ChargeRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> ChargeRepo<'a> {
    pub async fn insert(&self, charge: &Charge) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bill_charge(charge_id, encounter_id, patient_id, service_code, description, qty, unit_price, amount, status, charged_at, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"
        )
        .bind(charge.charge_id)
        .bind(charge.encounter_id)
        .bind(charge.patient_id)
        .bind(&charge.service_code)
        .bind(charge.description.as_ref())
        .bind(&charge.qty)
        .bind(&charge.unit_price)
        .bind(&charge.amount)
        .bind(&charge.status)
        .bind(charge.charged_at)
        .bind(charge.created_at)
        .bind(charge.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, charge_id: Uuid) -> anyhow::Result<Option<Charge>> {
        Ok(sqlx::query_as::<_, Charge>(
            "SELECT charge_id, encounter_id, patient_id, service_code, description, qty, unit_price, amount, status, charged_at, created_at, updated_at
             FROM bill_charge WHERE charge_id = $1"
        )
        .bind(charge_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_encounter(&self, encounter_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<Charge>> {
        Ok(sqlx::query_as::<_, Charge>(
            "SELECT charge_id, encounter_id, patient_id, service_code, description, qty, unit_price, amount, status, charged_at, created_at, updated_at
             FROM bill_charge WHERE encounter_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(encounter_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn list_by_patient(&self, patient_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<Charge>> {
        Ok(sqlx::query_as::<_, Charge>(
            "SELECT charge_id, encounter_id, patient_id, service_code, description, qty, unit_price, amount, status, charged_at, created_at, updated_at
             FROM bill_charge WHERE patient_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn update_status(&self, charge_id: Uuid, status: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE bill_charge SET status = $2, updated_at = NOW() WHERE charge_id = $1")
            .bind(charge_id)
            .bind(status)
            .execute(self.db)
            .await?;
        Ok(())
    }

    pub async fn list_pending_by_encounter(&self, encounter_id: Uuid) -> anyhow::Result<Vec<Charge>> {
        Ok(sqlx::query_as::<_, Charge>(
            "SELECT charge_id, encounter_id, patient_id, service_code, description, qty, unit_price, amount, status, charged_at, created_at, updated_at
             FROM bill_charge WHERE encounter_id = $1 AND status = 'PENDING' ORDER BY created_at"
        )
        .bind(encounter_id)
        .fetch_all(self.db)
        .await?)
    }
}

// Price List Repository
#[derive(Clone)]
pub struct PriceListRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> PriceListRepo<'a> {
    pub async fn insert(&self, price_list: &PriceList) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO price_list(price_list_id, facility_id, code, name, currency, valid_from, valid_to, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(price_list.price_list_id)
        .bind(price_list.facility_id)
        .bind(&price_list.code)
        .bind(&price_list.name)
        .bind(&price_list.currency)
        .bind(price_list.valid_from)
        .bind(price_list.valid_to)
        .bind(price_list.created_at)
        .bind(price_list.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, price_list_id: Uuid) -> anyhow::Result<Option<PriceList>> {
        Ok(sqlx::query_as::<_, PriceList>(
            "SELECT price_list_id, facility_id, code, name, currency, valid_from, valid_to, created_at, updated_at
             FROM price_list WHERE price_list_id = $1"
        )
        .bind(price_list_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_facility(&self, facility_id: Uuid) -> anyhow::Result<Vec<PriceList>> {
        Ok(sqlx::query_as::<_, PriceList>(
            "SELECT price_list_id, facility_id, code, name, currency, valid_from, valid_to, created_at, updated_at
             FROM price_list WHERE facility_id = $1 ORDER BY created_at DESC"
        )
        .bind(facility_id)
        .fetch_all(self.db)
        .await?)
    }
}

// Price Item Repository
#[derive(Clone)]
pub struct PriceItemRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> PriceItemRepo<'a> {
    pub async fn insert(&self, price_item: &PriceItem) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO price_item(price_item_id, price_list_id, service_code, description, unit_price, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(price_item.price_item_id)
        .bind(price_item.price_list_id)
        .bind(&price_item.service_code)
        .bind(price_item.description.as_ref())
        .bind(&price_item.unit_price)
        .bind(price_item.created_at)
        .bind(price_item.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, price_item_id: Uuid) -> anyhow::Result<Option<PriceItem>> {
        Ok(sqlx::query_as::<_, PriceItem>(
            "SELECT price_item_id, price_list_id, service_code, description, unit_price, created_at, updated_at
             FROM price_item WHERE price_item_id = $1"
        )
        .bind(price_item_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_price_list(&self, price_list_id: Uuid) -> anyhow::Result<Vec<PriceItem>> {
        Ok(sqlx::query_as::<_, PriceItem>(
            "SELECT price_item_id, price_list_id, service_code, description, unit_price, created_at, updated_at
             FROM price_item WHERE price_list_id = $1 ORDER BY service_code"
        )
        .bind(price_list_id)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn get_by_service_code(&self, price_list_id: Uuid, service_code: &str) -> anyhow::Result<Option<PriceItem>> {
        Ok(sqlx::query_as::<_, PriceItem>(
            "SELECT price_item_id, price_list_id, service_code, description, unit_price, created_at, updated_at
             FROM price_item WHERE price_list_id = $1 AND service_code = $2"
        )
        .bind(price_list_id)
        .bind(service_code)
        .fetch_optional(self.db)
        .await?)
    }
}
