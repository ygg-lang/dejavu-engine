use crate::{
    exports::dejavu::core::backends::{
        DejavuError, DejavuRoot, GuestJavaScriptVanilla, GuestRustDejavu, GuestRustVanilla, GuestTypeScriptVanilla,
    },
    DejavuHost,
};

impl crate::exports::dejavu::core::backends::Guest for DejavuHost {
    type RustVanilla = RsVanilla;
    type RustDejavu = RsDejavu;
    type JavaScriptVanilla = JsVanilla;
    type TypeScriptVanilla = TsVanilla;
}
