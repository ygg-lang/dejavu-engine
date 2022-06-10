use super::*;

impl ParserContext {
    pub fn left_destroyer(&self, mode: impl Into<SlotL>, statement: bool) -> SahaNode {
        let l = mode.into();
        SahaNode {
            kind: ASTKind::LeftDestroyer(SpaceDestroyer::new(l.trim, statement)),
            span: Default::default(),
            file: self.file.clone(),
        }
    }
    pub fn right_destroyer(&self, mode: impl Into<SlotR>, statement: bool) -> SahaNode {
        let r = mode.into();
        SahaNode {
            kind: ASTKind::RightDestroyer(SpaceDestroyer::new(r.trim, statement)),
            span: Default::default(),
            file: self.file.clone(),
        }
    }
}

impl From<&SlotL> for SlotL {
    fn from(value: &SlotL) -> Self {
        value.clone()
    }
}

impl From<&CommentL> for SlotL {
    fn from(value: &CommentL) -> Self {
        SlotL { trim: value.trim }
    }
}

impl From<&SlotR> for SlotR {
    fn from(value: &SlotR) -> Self {
        value.clone()
    }
}
impl From<&CommentR> for SlotR {
    fn from(value: &CommentR) -> Self {
        SlotR { trim: value.trim }
    }
}
