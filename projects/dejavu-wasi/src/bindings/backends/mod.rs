use crate::{
    backends::{JsVanilla, RsDejavu, RsVanilla, TsVanilla},
    DejavuHost,
};

impl crate::exports::dejavu::core::backends::Guest for DejavuHost {
    type RustVanilla = RsVanilla;
    type RustDejavu = RsDejavu;
    type JavaScriptVanilla = JsVanilla;
    type TypeScriptVanilla = TsVanilla;
}
