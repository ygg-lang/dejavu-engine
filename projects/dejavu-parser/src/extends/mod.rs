use crate::{CommentL, CommentR, IdentifierNode, NamespaceNode, SlotL, SlotR, TermNode, ValueNode};

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
        match &self.term {
            ValueNode::NamespaceNode(v) => v.is_normal_name(),
            _ => true,
        }
    }
}

impl NamespaceNode {
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
    fn is_normal_name(&self) -> bool {
        if self.path.len() != 1 {
            return true;
        }
        unsafe { self.path.get_unchecked(0).is_normal_name() }
    }
}

impl IdentifierNode {
    fn is_normal_name(&self) -> bool {
        !matches!(self.string.as_str(), "else" | "end" | "endfor" | "end-for" | "endif" | "end-if")
    }
}
