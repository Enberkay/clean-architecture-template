use std::sync::Arc;
use rust_decimal::prelude::FromPrimitive;

use crate::application::dtos::payment_dto::{
    CreatePaymentRequest,
    PaymentResponse,
};
use crate::domain::{
    entities::payment::PaymentEntity,
    repositories::{payment_repository::PaymentRepository, order_repository::OrderRepository},
    value_objects::money::Money,
};

/// PaymentService — handles payment creation, validation, and linking to orders
pub struct PaymentUseCase {
    payment_repo: Arc<dyn PaymentRepository>,
    order_repo: Arc<dyn OrderRepository>,
}

impl PaymentUseCase {
    pub fn new(
        payment_repo: Arc<dyn PaymentRepository>,
        order_repo: Arc<dyn OrderRepository>,
    ) -> Self {
        Self {
            payment_repo,
            order_repo,
        }
    }

    /// Create a new payment and mark related order as paid
    pub async fn create_payment(&self, req: CreatePaymentRequest) -> Result<PaymentResponse> {
        // Convert f64 → Money
        let amount = Money::from_decimal(
            rust_decimal::Decimal::from_f64(req.amount)
                .ok_or_else(|| anyhow::anyhow!("Invalid payment amount"))?,
        )?;

        let mut payment = PaymentEntity::new(
            req.order_id,
            req.sale_id,
            req.method.clone(),
            amount,
        )?;

        // If reference (transaction_ref) is provided, mark as paid
        if let Some(ref_ref) = &req.reference {
            payment.mark_paid(Some(ref_ref.clone()))?;
        }

        // Save payment
        self.payment_repo.save(&payment).await?;

        // If order exists, mark it as paid
        if let Some(order_id) = req.order_id {
            if let Some(mut order) = self.order_repo.find_by_id(order_id).await? {
                order.mark_paid()?;
                self.order_repo.save(&order).await?;
            }
        }

        Ok(PaymentResponse::from(payment))
    }

    /// Find payment by ID
    pub async fn get_payment_by_id(&self, id: i32) -> Result<Option<PaymentResponse>> {
        let payment_opt = self.payment_repo.find_by_id(id).await?;
        Ok(payment_opt.map(PaymentResponse::from))
    }

    /// Find all payments for a specific order
    pub async fn get_payments_by_order(&self, order_id: i32) -> Result<Vec<PaymentResponse>> {
        let payments = self.payment_repo.find_by_order(order_id).await?;
        Ok(payments.into_iter().map(PaymentResponse::from).collect())
    }
}
