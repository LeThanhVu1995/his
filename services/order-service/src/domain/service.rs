use uuid::Uuid;
use crate::domain::models::{Order, OrderItem};
use crate::domain::repo::{OrderRepo, ItemRepo};

pub struct OrderService<'a> {
    pub orders: OrderRepo<'a>,
    pub items: ItemRepo<'a>,
}

impl<'a> OrderService<'a> {
    pub async fn create_order(
        &self,
        req: &crate::dto::order_dto::CreateOrderReq,
        ordered_by: Option<&str>,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let order_no = format!("ORD-{}", &id.to_string()[..8]);
        let o = Order {
            id,
            patient_id: req.patient_id,
            encounter_id: req.encounter_id,
            order_no,
            order_type: req.order_type.clone(),
            status: "NEW".into(),
            priority: req.priority.clone(),
            ordered_by: ordered_by.map(|s| s.to_string()),
            note: req.note.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.orders.create(&o).await?;

        for it in &req.items {
            let item = OrderItem {
                id: Uuid::new_v4(),
                order_id: o.id,
                item_code: it.item_code.clone(),
                item_name: it.item_name.clone(),
                quantity: it.quantity.unwrap_or(1),
                status: "NEW".into(),
                result_json: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            self.items.add(&item).await?;
        }

        // TODO: Add event publishing when Kafka is available
        tracing::info!(order_id = %id, "Order created successfully");

        Ok(id)
    }
}
