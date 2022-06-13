use crate::parser::saha::{SlotExpressionNode, ValueNode};

pub fn unicode_text(input: &str) -> Result<(&str, usize), &'static str> {
    let mut length = 0;
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' => match chars.peek() {
                Some('%') => break,
                Some('#') => break,
                _ => length += c.len_utf8(),
            },
            _ => length += c.len_utf8(),
        }
    }
    if length == 0 { Err("Not unicode text") } else { Ok((&input[0..length], length)) }
}

pub fn check_slot_expression(slot: &SlotExpressionNode) -> bool {
    if !slot.e.infix.is_empty() {
        return true;
    }
    let id = match &slot.e.head {
        ValueNode::IdentifierNode(v) => v.string.as_str(),
        _ => return true,
    };
    !matches!(id, "else" | "end" | "endfor" | "end-for" | "end_for" | "endif" | "end-if" | "end_if")
}
