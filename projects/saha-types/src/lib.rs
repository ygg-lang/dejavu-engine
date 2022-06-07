pub use diagnostic_quick::{Failure, FileID, QError, QResult, Success, Validation};
pub use rust_decimal::Decimal;

pub use num::Zero;

pub use self::value::{for_statement::ForStatement, SahaNode, SahaValue, SpaceDestroyer};

// mod errors;

mod value;
