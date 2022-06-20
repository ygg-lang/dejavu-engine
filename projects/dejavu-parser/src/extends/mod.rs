use crate::{CommentL, CommentR, SlotL, SlotR, TermNode, ValueNode};

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

impl TermNode {
    /// 不要把 end 节点解析为 identifier
    pub(crate) fn is_normal_slot(&self) -> bool {
        if self.prefix.is_empty() || self.suffix.is_empty() {
            return true;
        }
        let id = match &self.term {
            ValueNode::IdentifierNode(v) => v.string.as_str(),
            _ => return true,
        };
        !matches!(id, "else" | "end" | "end-for" | "end-if")
    }
}
