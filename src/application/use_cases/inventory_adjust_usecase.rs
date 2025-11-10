use std::sync::Arc;

use crate::domain::{
    entities::{inventory::InventoryEntity, sale::SaleItemEntity},
    repositories::inventory_repository::InventoryRepository,
    value_objects::isbn13::Isbn13,
};

/// InventoryAdjustUseCase
/// ใช้ปรับปรุงสต็อกหลังการขาย เช่น ลดจำนวนสินค้าที่ขายออก
pub struct InventoryAdjustUseCase {
    inventory_repo: Arc<dyn InventoryRepository>,
}

impl InventoryAdjustUseCase {
    pub fn new(inventory_repo: Arc<dyn InventoryRepository>) -> Self {
        Self { inventory_repo }
    }

    /// ลด stock ตามรายการขายแต่ละเล่ม
    pub async fn apply_sale_items(
        &self,
        branch_id: i32,
        sale_items: &[SaleItemEntity],
    ) -> Result<()> {
        for item in sale_items {
            // แปลง string เป็น Isbn13 value object
            let isbn = Isbn13::new(&item.book_isbn)?;

            // ค้นหาสต็อกของสาขานี้
            let existing_opt = self.inventory_repo.find(branch_id, &isbn).await?;

            match existing_opt {
                // ถ้ามีสต็อกอยู่แล้ว → ลดตามจำนวน
                Some(mut inv) => {
                    inv.decrease(item.quantity as u32)?;
                    self.inventory_repo.update(&inv).await?;
                }

                // ถ้าไม่พบ → สร้างใหม่พร้อมค่าติดลบ (ถือว่า oversell)
                None => {
                    let mut new_inv = InventoryEntity::new(branch_id, isbn.clone(), 0)?;
                    new_inv.decrease(item.quantity as u32)?;
                    self.inventory_repo.save(&new_inv).await?;
                }
            }
        }

        Ok(())
    }

    /// ฟังก์ชันใช้คืนสินค้า (เพิ่มสต็อก)
    pub async fn revert_sale_items(
        &self,
        branch_id: i32,
        sale_items: &[SaleItemEntity],
    ) -> Result<()> {
        for item in sale_items {
            let isbn = Isbn13::new(&item.book_isbn)?;
            let existing_opt = self.inventory_repo.find(branch_id, &isbn).await?;

            if let Some(mut inv) = existing_opt {
                inv.increase(item.quantity as u32)?;
                self.inventory_repo.update(&inv).await?;
            } else {
                let new_inv = InventoryEntity::new(branch_id, isbn.clone(), item.quantity)?;
                self.inventory_repo.save(&new_inv).await?;
            }
        }

        Ok(())
    }
}
