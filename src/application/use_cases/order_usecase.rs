use std::sync::Arc;
use rust_decimal::prelude::FromPrimitive;

use crate::application::dtos::order_dto::{
    CreateOrderRequest,
    OrderResponse,
};
use crate::domain::{
    entities::order::{OrderEntity, OrderItemEntity},
    repositories::order_repository::OrderRepository,
    value_objects::money::Money,
};

/// OrderService — handles order orchestration and transaction logic
pub struct OrderUseCase {
    order_repo: Arc<dyn OrderRepository>,
}

impl OrderUseCase {
    pub fn new(order_repo: Arc<dyn OrderRepository>) -> Self {
        Self { order_repo }
    }

    /// Create a new order with its items
    pub async fn create_order(&self, req: CreateOrderRequest) -> Result<OrderResponse> {
        // 1. สร้าง OrderEntity
        let mut order = OrderEntity::new(
            Some(req.user_id),
            req.source.clone(),
            req.shipping_address.clone(),
        )?;

        // 2. สร้าง OrderItemEntity ทั้งหมด
        let mut items = Vec::new();
        for item in &req.items {
            let price = Money::from_decimal(
                rust_decimal::Decimal::from_f64(item.unit_price)
                    .ok_or_else(|| anyhow::anyhow!("Invalid price"))?,
            )?;

            let order_item = OrderItemEntity::new(
                order.id,              // order_id = 0 ก่อน insert
                item.book_isbn.clone(), // String
                item.book_title.clone(),
                item.book_author.clone(),
                item.quantity,
                price,
            )?;

            items.push(order_item);
        }

        // 3. คำนวณยอดรวมทั้งหมด
        let total = items.iter().fold(Money::zero(), |acc, i| acc.add(i.subtotal.clone()));
        order.update_total(total)?;

        // 4. บันทึกทั้ง order + items (transactional)
        self.order_repo.save_with_items(&order, &items).await?;

        Ok(OrderResponse::from((order, items)))
    }

    /// Get order with its items by ID
    pub async fn get_order_by_id(&self, id: i32) -> Result<Option<OrderResponse>> {
        let order_opt = self.order_repo.find_by_id(id).await?;
        if let Some(order) = order_opt {
            let items = self.order_repo.find_items(order.id).await?;
            Ok(Some(OrderResponse::from((order, items))))
        } else {
            Ok(None)
        }
    }

    /// Delete order and its items
    pub async fn delete_order(&self, id: i32) -> Result<()> {
        self.order_repo.delete(id).await
    }

    /// Mark order as paid
    pub async fn mark_as_paid(&self, id: i32) -> Result<()> {
        let mut order = match self.order_repo.find_by_id(id).await? {
            Some(o) => o,
            None => anyhow::bail!("Order not found"),
        };
        order.mark_paid()?;
        self.order_repo.save(&order).await
    }
}
