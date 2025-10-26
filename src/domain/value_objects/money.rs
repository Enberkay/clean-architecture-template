#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Money(f64);

impl Money {
    /// Create a new Money value. Must be >= 0 and finite.
    pub fn new(value: f64) -> Result<Self, String> {
        if !value.is_finite() {
            return Err("Money must be a finite number".into());
        }
        if value < 0.0 {
            return Err("Money cannot be negative".into());
        }

        // round to 2 decimals for currency precision
        let rounded = (value * 100.0).round() / 100.0;
        Ok(Self(rounded))
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
    pub fn subtract(self, other: Money) -> Result<Money, String> {
        let result = self.0 - other.0;
        if result < 0.0 {
            return Err("Resulting money cannot be negative".into());
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
