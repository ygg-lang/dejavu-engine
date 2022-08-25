use bigdecimal::BigDecimal;
use num::FromPrimitive;
use serde::{Deserialize, Serialize};

mod primitive;

/// An arbitrary-precision decimal with a unit
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decimal {
    /// The unit of the decimal
    pub hint: String,
    /// The value of the decimal
    pub value: BigDecimal,
}
