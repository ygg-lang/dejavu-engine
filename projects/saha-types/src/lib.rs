pub use diagnostic_quick::{Failure, FileID, QError, QResult, Success, Validation};
pub use num::Zero;
pub use rust_decimal::Decimal;

pub use self::{
    compiler::{render::SahaRender, SahaCompiler},
    value::{for_statement::ForStatement, SahaNode, SahaValue, SpaceDestroyer},
};

// mod errors;

mod compiler;
pub mod utils;
mod value;
