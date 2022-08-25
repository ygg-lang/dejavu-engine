use num::{FromPrimitive, ToPrimitive};

use super::*;

impl FromPrimitive for Number {
    fn from_i64(n: i64) -> Option<Self> {
        let value = BigDecimal::from_i64(n)?;
        Some(Number { hint: "".to_string(), value })
    }

    fn from_i128(n: i128) -> Option<Self> {
        let value = BigDecimal::from_i128(n)?;
        Some(Number { hint: "".to_string(), value })
    }

    fn from_u64(n: u64) -> Option<Self> {
        let value = BigDecimal::from_u64(n)?;
        Some(Number { hint: "".to_string(), value })
    }

    fn from_u128(n: u128) -> Option<Self> {
        let value = BigDecimal::from_u128(n)?;
        Some(Number { hint: "".to_string(), value })
    }

    fn from_f64(n: f64) -> Option<Self> {
        let value = BigDecimal::from_f64(n)?;
        Some(Number { hint: "".to_string(), value })
    }
}

impl ToPrimitive for Number {
    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    fn to_i128(&self) -> Option<i128> {
        self.value.to_i128()
    }
    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }
    fn to_u128(&self) -> Option<u128> {
        self.value.to_u128()
    }

    fn to_f64(&self) -> Option<f64> {
        self.value.to_f64()
    }
}
