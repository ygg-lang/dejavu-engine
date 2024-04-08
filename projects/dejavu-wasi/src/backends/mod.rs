use crate::{
    exports::dejavu::core::{
        backends::{DejavuError, GuestJavaScriptVanilla, GuestRustDejavu, GuestRustVanilla, GuestTypeScriptVanilla},
        syntax_tree::{DejavuTemplate, TemplateItem, TextElement},
    },
    traits::RsDejavuBuilder,
};
pub use build_rust::RsDejavu;
use indentation::IndentFormatter;
use std::fmt::Write;

mod build_rust;

pub struct RsVanilla {}

impl GuestRustVanilla for RsVanilla {
    fn new(directory: String) -> Self {
        todo!()
    }

    fn finalize(&self, ast: DejavuTemplate) -> Result<(), DejavuError> {
        todo!()
    }
}

pub struct JsVanilla {}

impl GuestJavaScriptVanilla for JsVanilla {
    fn new(directory: String) -> Self {
        todo!()
    }

    fn finalize(&self, ast: DejavuTemplate) -> Result<(), DejavuError> {
        todo!()
    }
}

pub struct TsVanilla {}

impl GuestTypeScriptVanilla for TsVanilla {
    fn new(directory: String) -> Self {
        todo!()
    }

    fn finalize(&self, ast: DejavuTemplate) -> Result<(), DejavuError> {
        todo!()
    }
}
