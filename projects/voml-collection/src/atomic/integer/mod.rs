use num::BigInt;
use serde::{Deserialize, Serialize};

mod primitive;

/// An arbitrary-precision integer with a unit
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Integer {
    /// The unit of this integer
    pub hint: String,
    /// The value of this integer
    pub value: BigInt,
}
