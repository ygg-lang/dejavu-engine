pub use diagnostic_quick::{Failure, FileID, QError, QResult, Success, Validation};
pub use num::Zero;
pub use rust_decimal::Decimal;

pub use self::{
    compiler::{render::SahaRenderer, SahaVM},
    value::{for_statement::ForStatement, ser::SahaSerializer, ASTKind, SahaNode, SpaceDestroyer},
};

// mod escaper;

mod compiler;
pub mod utils;
mod value;
