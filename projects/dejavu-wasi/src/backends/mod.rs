use crate::exports::dejavu::core::backends::{
    DejavuError, DejavuRoot, GuestJavaScriptVanilla, GuestRustDejavu, GuestRustVanilla, GuestTypeScriptVanilla,
};

mod build_rust;

pub struct RsVanilla {}

impl GuestRustVanilla for RsVanilla {
    fn new(directory: String) -> Self {
        todo!()
    }

    fn finalize(&self, ast: DejavuRoot) -> Result<(), DejavuError> {
        todo!()
    }
}

pub struct JsVanilla {}

impl GuestJavaScriptVanilla for JsVanilla {
    fn new(directory: String) -> Self {
        todo!()
    }

    fn finalize(&self, ast: DejavuRoot) -> Result<(), DejavuError> {
        todo!()
    }
}

pub struct TsVanilla {}

impl GuestTypeScriptVanilla for TsVanilla {
    fn new(directory: String) -> Self {
        todo!()
    }

    fn finalize(&self, ast: DejavuRoot) -> Result<(), DejavuError> {
        todo!()
    }
}
