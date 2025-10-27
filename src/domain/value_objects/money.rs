use crate::domain::domain_errors::{DomainError, DomainResult};
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Money(f64);

impl Money {
    /// Create a new Money value. Must be >= 0 and finite.
    pub fn new(value: f64) -> DomainResult<Self> {
        if !value.is_finite() {
            return Err(DomainError::validation("Money must be a finite number"));
        }
        if value < 0.0 {
            return Err(DomainError::validation("Money cannot be negative"));
        }

        // Round to 2 decimals for currency precision
        let rounded = (value * 100.0).round() / 100.0;
        Ok(Self(rounded))
    }

    /// Create a Money value of zero
    pub fn zero() -> Self {
        Self(0.0)
    }

    /// Get the numeric value
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Add two Money values
    pub fn add(self, other: Money) -> Money {
        Money((self.0 + other.0).round_to_2dp())
    }

    /// Subtract other Money (error if result < 0)
    pub fn subtract(self, other: Money) -> DomainResult<Money> {
        let result = self.0 - other.0;
        if result < 0.0 {
            return Err(DomainError::validation(
                "Resulting money cannot be negative",
            ));
        }
        Ok(Money(result.round_to_2dp()))
    }

    /// Multiply by a scalar (e.g. quantity)
    pub fn multiply(self, qty: u32) -> Money {
        Money((self.0 * qty as f64).round_to_2dp())
    }
}

/// Helper trait for rounding cleanly
trait RoundTo2Dp {
    fn round_to_2dp(self) -> f64;
}

impl RoundTo2Dp for f64 {
    fn round_to_2dp(self) -> f64 {
        (self * 100.0).round() / 100.0
    }
}

/// Allow Money * f64 operator syntax
impl Mul<f64> for Money {
    type Output = Money;

    fn mul(self, rhs: f64) -> Self::Output {
        Money((self.0 * rhs).round_to_2dp())
    }
}
