use std::sync::Arc;
use anyhow::Result;
use rust_decimal::prelude::FromPrimitive;

use crate::application::dtos::receipt_dto::{
    CreateReceiptRequest,
    ReceiptResponse,
};
use crate::domain::{
    entities::receipt::ReceiptEntity,
    repositories::{receipt_repository::ReceiptRepository, payment_repository::PaymentRepository},
    value_objects::{money::Money, receipt_code::ReceiptCode},
};

/// ReceiptService — orchestrates receipt creation, validation, and cancellation
pub struct ReceiptService {
    receipt_repo: Arc<dyn ReceiptRepository>,
    payment_repo: Arc<dyn PaymentRepository>,
}

impl ReceiptService {
    pub fn new(
        receipt_repo: Arc<dyn ReceiptRepository>,
        payment_repo: Arc<dyn PaymentRepository>,
    ) -> Self {
        Self {
            receipt_repo,
            payment_repo,
        }
    }

    /// Create a new receipt — only if payment is already completed
    pub async fn create_receipt(&self, req: CreateReceiptRequest) -> Result<ReceiptResponse> {
        // 1. ตรวจสอบการชำระเงินก่อนออกใบเสร็จ
        let payments = self.payment_repo.find_by_order(req.reference_id).await?;
        let has_paid = payments.iter().any(|p| p.is_paid());

        if !has_paid {
            anyhow::bail!("Cannot issue receipt before payment is completed");
        }

        // 2. แปลง f64 → Money
        let total_amount = Money::from_decimal(
            rust_decimal::Decimal::from_f64(req.total_amount)
                .ok_or_else(|| anyhow::anyhow!("Invalid total amount"))?,
        )?;

        // 3. แปลง receipt code จาก string
        let receipt_code = ReceiptCode::new(&req.receipt_code)?;

        // 4. สร้าง entity
        let mut receipt = ReceiptEntity::new(
            receipt_code,
            req.type_name.clone(),
            req.reference_id,
            req.source.clone(),
            total_amount,
            req.user_id,
            req.branch_id,
            req.payment_method.clone(),
        )?;

        // 5. ถ้ามี payment_ref จาก request หรือจาก payment ล่าสุดให้แนบ
        let latest_payment_ref = payments
            .iter()
            .filter(|p| p.is_paid())
            .last()
            .and_then(|p| p.transaction_ref.clone());

        if let Some(ref_code) = latest_payment_ref.or(req.payment_ref.clone()) {
            receipt.set_payment_ref(Some(ref_code));
        }

        // 6. บันทึกใบเสร็จ
        self.receipt_repo.save(&receipt).await?;

        Ok(ReceiptResponse::from(receipt))
    }

    /// Find receipt by code
    pub async fn get_receipt_by_code(&self, code: &str) -> Result<Option<ReceiptResponse>> {
        let receipt_code = ReceiptCode::new(code)?;
        let receipt_opt = self.receipt_repo.find_by_code(&receipt_code).await?;
        Ok(receipt_opt.map(ReceiptResponse::from))
    }

    /// Find receipts by reference (order_id / sale_id)
    pub async fn get_receipts_by_reference(&self, reference_id: i32) -> Result<Vec<ReceiptResponse>> {
        let receipts = self.receipt_repo.find_by_reference(reference_id).await?;
        Ok(receipts.into_iter().map(ReceiptResponse::from).collect())
    }

    /// Cancel a receipt
    pub async fn cancel_receipt(&self, code: &str) -> Result<()> {
        let receipt_code = ReceiptCode::new(code)?;
        let mut receipt = match self.receipt_repo.find_by_code(&receipt_code).await? {
            Some(r) => r,
            None => anyhow::bail!("Receipt not found"),
        };

        receipt.mark_cancelled()?;
        self.receipt_repo.save(&receipt).await
    }
}
