use std::sync::Arc;
use rust_decimal::prelude::FromPrimitive;
use chrono::Utc;

use crate::application::{
    dtos::sale_dto::{CreateSaleRequest, SaleResponse, SaleItemRequest},
    use_cases::inventory_adjust_usecase::InventoryAdjustUseCase,
};
use crate::domain::{
    entities::{
        sale::{SaleEntity, SaleItemEntity},
        payment::PaymentEntity,
        receipt::ReceiptEntity,
    },
    repositories::{
        sale_repository::SaleRepository,
        payment_repository::PaymentRepository,
        receipt_repository::ReceiptRepository,
    },
    value_objects::{money::Money, receipt_code::ReceiptCode},
};

/// SaleService — orchestrates sale → payment → receipt → inventory flow
pub struct SaleUseCase {
    sale_repo: Arc<dyn SaleRepository>,
    payment_repo: Arc<dyn PaymentRepository>,
    receipt_repo: Arc<dyn ReceiptRepository>,
    inventory_adjust_service: Arc<InventoryAdjustUseCase>, //เชื่อม inventory service
}

impl SaleUseCase {
    pub fn new(
        sale_repo: Arc<dyn SaleRepository>,
        payment_repo: Arc<dyn PaymentRepository>,
        receipt_repo: Arc<dyn ReceiptRepository>,
        inventory_adjust_service: Arc<InventoryAdjustUseCase>,
    ) -> Self {
        Self {
            sale_repo,
            payment_repo,
            receipt_repo,
            inventory_adjust_service,
        }
    }

    /// Create a new sale, process payment, issue receipt, and adjust inventory
    pub async fn create_sale(&self, req: CreateSaleRequest) -> Result<SaleResponse> {
        // Step 1: Convert f64 → Money
        let total = Money::from_decimal(
            rust_decimal::Decimal::from_f64(req.total_amount)
                .ok_or_else(|| anyhow::anyhow!("Invalid total amount"))?,
        )?;

        // Step 2: Create SaleEntity
        let sale = SaleEntity::new(
            Some(req.user_id),
            Some(req.branch_id),
            req.payment_method.clone(),
        )?;

        // Save sale record
        self.sale_repo.save(&sale).await?;

        // Step 3: Create PaymentEntity
        let mut payment = PaymentEntity::new(
            None,                      // no order_id
            Some(sale.id),             // link to sale
            req.payment_method.clone(),
            total.clone(),
        )?;

        // Mark paid (simulate success)
        payment.mark_paid(Some(format!("TXN-{}", Utc::now().timestamp())))?;
        self.payment_repo.save(&payment).await?;

        // Step 4: Create ReceiptEntity
        let receipt_code = ReceiptCode::generate("RC", Utc::now().timestamp() as u32)?;
        let mut receipt = ReceiptEntity::new(
            receipt_code,
            "SALE".to_string(),
            sale.id,
            "POS".to_string(),
            total.clone(),
            Some(req.user_id),
            Some(req.branch_id),
            Some(req.payment_method.clone()),
        )?;

        receipt.set_payment_ref(payment.transaction_ref.clone());
        self.receipt_repo.save(&receipt).await?;

        //Step 5: Convert DTO items → domain entities
        let sale_items: Vec<SaleItemEntity> = req
            .items
            .iter()
            .map(|i: &SaleItemRequest| {
                let unit_price = rust_decimal::Decimal::from_f64(i.unit_price)
                    .ok_or_else(|| anyhow::anyhow!("Invalid unit price"))?;
                let price_money = Money::from_decimal(unit_price)?;
                Ok(SaleItemEntity::new(
                    sale.id,
                    i.book_isbn.clone(),
                    i.book_title.clone(),
                    i.book_author.clone(),
                    i.quantity,
                    price_money,
                )?)
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        //Step 6: Adjust inventory (หักสต็อกตามสินค้าที่ขาย)
        self.inventory_adjust_service
            .apply_sale_items(req.branch_id, &sale_items)
            .await?;

        // Step 7: Return composed response
        Ok(SaleResponse::from((sale, payment, receipt)))
    }

    /// Retrieve sale by ID (with payment + receipt)
    pub async fn get_sale_by_id(&self, id: i32) -> Result<Option<SaleResponse>> {
        let sale_opt = self.sale_repo.find_by_id(id).await?;
        let Some(sale) = sale_opt else {
            return Ok(None);
        };

        let payment_opt = self
            .payment_repo
            .find_by_order(id)
            .await?
            .into_iter()
            .find(|p| p.sale_id == Some(id));

        let receipt_opt = self
            .receipt_repo
            .find_by_reference(id)
            .await?
            .into_iter()
            .find(|r| r.reference_id == id);

        Ok(Some(SaleResponse::compose(sale, payment_opt, receipt_opt)))
    }
}
