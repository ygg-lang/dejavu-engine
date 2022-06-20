use crate::{SlotExpressionNode, ValueNode};

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
    slot.e.head.is_normal_slot()
}
