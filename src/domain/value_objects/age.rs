use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Age(i32);

impl Age {
    pub fn new(age: i32) -> Result<Self> {
        if age < 1 || age > 120 {
            return Err(anyhow!("Age must be between 1 and 120"));
        }
        Ok(Self(age))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}