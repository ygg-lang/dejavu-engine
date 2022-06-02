use super::*;

impl ParserContext {
    pub fn left_destroyer(&self, mode: impl Into<SlotL>, statement: bool) -> SahaNode {
        SahaNode { kind: SahaValue::LeftDestroyer(SpaceDestroyer::new(mode.into().trim, statement)), span: Default::default() }
    }
    pub fn right_destroyer(&self, mode: impl Into<SlotR>, statement: bool) -> SahaNode {
        SahaNode { kind: SahaValue::RightDestroyer(SpaceDestroyer::new(mode.into().trim, statement)), span: Default::default() }
    }
}

impl From<&SlotL> for SlotL {
    fn from(value: &SlotL) -> Self {
        value.into()
    }
}

impl From<&CommentL> for SlotL {
    fn from(value: &CommentL) -> Self {
        SlotL { trim: value.trim }
    }
}

impl From<&SlotR> for SlotR {
    fn from(value: &SlotR) -> Self {
        value.into()
    }
}
impl From<&CommentR> for SlotR {
    fn from(value: &CommentR) -> Self {
        SlotR { trim: value.trim }
    }
}
