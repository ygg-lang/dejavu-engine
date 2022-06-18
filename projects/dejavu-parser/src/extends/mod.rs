use crate::{CommentL, CommentR, SlotL, SlotR};

impl From<&SlotL> for SlotL {
    fn from(value: &SlotL) -> Self {
        value.clone()
    }
}

impl From<&CommentL> for SlotL {
    fn from(value: &CommentL) -> Self {
        SlotL { trim: value.trim, position: value.position.clone() }
    }
}

impl From<&SlotR> for SlotR {
    fn from(value: &SlotR) -> Self {
        value.clone()
    }
}

impl From<&CommentR> for SlotR {
    fn from(value: &CommentR) -> Self {
        SlotR { trim: value.trim, position: value.position.clone() }
    }
}
