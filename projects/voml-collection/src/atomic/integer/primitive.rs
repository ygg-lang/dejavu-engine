use super::*;
use num::{FromPrimitive, ToPrimitive};

impl FromPrimitive for Integer {
    fn from_i64(_n: i64) -> Option<Self> {
        todo!()
    }

    fn from_u64(_n: u64) -> Option<Self> {
        todo!()
    }
}

impl ToPrimitive for Integer {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}
