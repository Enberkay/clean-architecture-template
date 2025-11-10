use anyhow::{Result, anyhow};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(Decimal);

impl Money {
    /// Create a new Money value. Must be >= 0
    pub fn new(value: Decimal) -> Result<Self> {
        if value.is_sign_negative() {
            return Err(anyhow!("Money cannot be negative"));
        }
        Ok(Self(value.round_dp(2)))
    }

    /// Create a Money value of zero
    pub fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    /// Get the numeric value
    pub fn value(&self) -> Decimal {
        self.0
    }

    /// Convert from Decimal (used in models)
    pub fn from_decimal(value: Decimal) -> Result<Self> {
        Self::new(value)
    }

    /// Convert Money → Decimal (for database persistence)
    pub fn to_decimal(&self) -> Decimal {
        self.0
    }

    /// Add two Money values
    pub fn add(self, other: Money) -> Money {
        Money((self.0 + other.0).round_dp(2))
    }

    /// Subtract other Money (error if result < 0)
    pub fn subtract(self, other: Money) -> Result<Money> {
        let result = self.0 - other.0;
        if result.is_sign_negative() {
            return Err(anyhow!("Resulting money cannot be negative"));
        }
        Ok(Money(result.round_dp(2)))
    }

    /// Multiply by a scalar (e.g. quantity)
    pub fn multiply(self, qty: u32) -> Money {
        Money((self.0 * Decimal::from(qty)).round_dp(2))
    }

    /// Convert from f64 to Money (for tests)
    pub fn from_f64(value: f64) -> Result<Self> {
        Decimal::try_from(value)
            .map_err(|_| anyhow!("Invalid float for Decimal"))
            .and_then(Money::new)
    }

    /// Convert Money to f64 (for display)
    pub fn to_f64(&self) -> f64 {
        self.0.to_f64().unwrap_or(0.0)
    }
}

/// Allow Money * f64 syntax
impl Mul<f64> for Money {
    type Output = Money;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs_dec = Decimal::try_from(rhs).unwrap_or(Decimal::ZERO);
        Money((self.0 * rhs_dec).round_dp(2))
    }
}
