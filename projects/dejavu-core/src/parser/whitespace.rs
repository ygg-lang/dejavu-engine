use super::*;
use crate::value::ASTKind;
use dejavu_parser::{SlotL, SlotR};

impl ParserContext {
    pub fn left_destroyer(&self, mode: impl Into<SlotL>, statement: bool) -> SahaNode {
        let l = mode.into();
        SahaNode {
            kind: ASTKind::LeftDestroyer(SpaceDestroyer::new(l.trim, statement)),
            span: l.position,
            file: self.file.clone(),
        }
    }
    pub fn right_destroyer(&self, mode: impl Into<SlotR>, statement: bool) -> SahaNode {
        let r = mode.into();
        SahaNode {
            kind: ASTKind::RightDestroyer(SpaceDestroyer::new(r.trim, statement)),
            span: r.position,
            file: self.file.clone(),
        }
    }
}
