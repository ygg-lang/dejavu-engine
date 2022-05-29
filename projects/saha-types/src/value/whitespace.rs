use std::fmt::Write;

use super::*;

impl Display for SpaceDestroyer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceDestroyer::Everything => f.write_char('='),
            SpaceDestroyer::NewlineAll => f.write_char('-'),
            SpaceDestroyer::NewlineOne => f.write_char('_'),
            SpaceDestroyer::Nothing => f.write_char('!'),
        }
    }
}

impl SpaceDestroyer {
    pub fn new(c: Option<char>, statement: bool) -> Self {
        match c {
            Some('=') => SpaceDestroyer::Everything,
            Some('-') => SpaceDestroyer::NewlineAll,
            Some('_') => SpaceDestroyer::NewlineOne,
            Some('!') => SpaceDestroyer::Nothing,
            _ if statement => SpaceDestroyer::NewlineOne,
            _ => SpaceDestroyer::Nothing,
        }
    }
}

impl SpaceDestroyer {
    /// Clear all space destroyer
    pub fn clear(list: Vec<SahaNode>) -> Vec<SahaNode> {
        let mut out = Vec::with_capacity(list.len());
        let mut iter = list.into_iter().peekable();
        while let Some(s) = iter.next() {
            match &s.kind {
                SahaValue::Text(text) => match iter.peek() {
                    Some(next) => match &next.kind {
                        SahaValue::LeftDestroyer(ws) => match ws.trim_end(text) {
                            // drop node
                            "" => {}
                            str => out.push(SahaNode::text(str).with_span(s.span)),
                        },
                        _ => out.push(s),
                    },
                    None => out.push(s),
                },
                // drop node
                SahaValue::LeftDestroyer(_) => {}
                SahaValue::RightDestroyer(s) => {
                    if let Some(next) = iter.next() {
                        match &next.kind {
                            SahaValue::Text(text) => {
                                match s.trim_start(text) {
                                    // drop
                                    "" => {}
                                    str => out.push(SahaNode::text(str).with_span(next.span)),
                                }
                            }
                            _ => out.push(next),
                        }
                    }
                }
                _ => out.push(s),
            }
        }
        out
    }
    pub fn trim_end<'i>(&self, input: &'i str) -> &'i str {
        let mut length = 0;
        match self {
            SpaceDestroyer::Everything => input.trim_end(),
            SpaceDestroyer::NewlineAll => {
                let mut line_buffer = 0;
                for char in input.chars() {
                    if char.is_whitespace() {
                        line_buffer += char.len_utf8();
                        match char {
                            '\r' | '\n' => break,
                            _ => continue,
                        }
                    }
                    else {
                        break;
                    }
                }
                &input[0..length]
            }
            SpaceDestroyer::NewlineOne => {
                for char in input.chars().rev() {
                    if char.is_whitespace() {
                        length += char.len_utf8();
                        match char {
                            '\r' | '\n' => break,
                            _ => continue,
                        }
                    }
                    else {
                        break;
                    }
                }
                &input[0..length]
            }
            SpaceDestroyer::Nothing => input,
        }
    }
    pub fn trim_start<'i>(&self, input: &'i str) -> &'i str {
        let all = input.len();
        let mut length = 0;
        match self {
            SpaceDestroyer::Everything => input.trim_start(),
            SpaceDestroyer::NewlineAll => input,
            SpaceDestroyer::NewlineOne => {
                for char in input.chars() {
                    if char.is_whitespace() {
                        length += char.len_utf8();
                        match char {
                            '\r' | '\n' => break,
                            _ => continue,
                        }
                    }
                    else {
                        break;
                    }
                }
                &input[length..all]
            }
            SpaceDestroyer::Nothing => input,
        }
    }
}
