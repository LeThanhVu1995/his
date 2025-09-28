use anyhow::Result;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::entities::order::ClinicalOrder;
use crate::infra::db::repositories::OrderRepo;
use crate::http::dto::order::*;

pub struct OrderService<'a> {
    repo: OrderRepo<'a>,
}

impl<'a> OrderService<'a> {
    pub fn new(repo: OrderRepo<'a>) -> Self {
        Self { repo }
    }

    // Clinical Order CRUD operations
    pub async fn create_order(&self, req: CreateOrderRequest) -> Result<ClinicalOrder> {
        let order_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let order = ClinicalOrder {
            order_id: order_id.clone(),
            encounter_id: req.encounter_id,
            patient_id: req.patient_id,
            order_type: req.order_type,
            status: req.status.unwrap_or_else(|| "pending".to_string()),
            ordered_by: req.ordered_by,
            ordered_at: req.ordered_at,
            priority_code: req.priority_code,
            remarks: req.remarks,
        };

        self.repo.create_order(&order).await?;
        Ok(order)
    }

    pub async fn get_order(&self, order_id: &str) -> Result<Option<ClinicalOrder>> {
        self.repo.get_order(order_id).await
    }

    pub async fn list_patient_orders(
        &self,
        patient_id: &str,
        order_type: Option<&str>,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<ClinicalOrder>, i64)> {
        let orders = self.repo.list_patient_orders(patient_id, limit, offset).await?;
        let total = self.repo.count_patient_orders(patient_id).await?;
        Ok((orders, total))
    }

    pub async fn list_encounter_orders(
        &self,
        encounter_id: &str,
        order_type: Option<&str>,
        status: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<ClinicalOrder>, i64)> {
        let orders = self.repo.list_encounter_orders(encounter_id, limit, offset).await?;
        let total = self.repo.count_encounter_orders(encounter_id).await?;
        Ok((orders, total))
    }

    pub async fn update_order(&self, order_id: &str, req: UpdateOrderRequest) -> Result<ClinicalOrder> {
        let mut order = self.repo.get_order(order_id).await?
            .ok_or_else(|| anyhow::anyhow!("Order not found"))?;

        if let Some(order_type) = req.order_type {
            order.order_type = order_type;
        }
        if let Some(status) = req.status {
            order.status = status;
        }
        if let Some(ordered_by) = req.ordered_by {
            order.ordered_by = Some(ordered_by);
        }
        if let Some(ordered_at) = req.ordered_at {
            order.ordered_at = ordered_at;
        }
        if let Some(priority_code) = req.priority_code {
            order.priority_code = Some(priority_code);
        }
        if let Some(remarks) = req.remarks {
            order.remarks = Some(remarks);
        }

        self.repo.update_order(&order).await?;
        Ok(order)
    }

    pub async fn complete_order(&self, order_id: &str, user_id: &str) -> Result<ClinicalOrder> {
        let mut order = self.repo.get_order(order_id).await?
            .ok_or_else(|| anyhow::anyhow!("Order not found"))?;

        order.status = "completed".to_string();

        self.repo.update_order(&order).await?;
        Ok(order)
    }

    pub async fn cancel_order(&self, order_id: &str, user_id: &str) -> Result<()> {
        let mut order = self.repo.get_order(order_id).await?
            .ok_or_else(|| anyhow::anyhow!("Order not found"))?;

        order.status = "cancelled".to_string();

        self.repo.update_order(&order).await?;
        Ok(())
    }

    pub async fn delete_order(&self, order_id: &str, user_id: &str) -> Result<()> {
        self.repo.delete_order(order_id, user_id).await
    }
}
