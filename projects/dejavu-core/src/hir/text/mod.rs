use super::*;
use alloc::string::ToString;

#[derive(Debug, Default)]
pub struct DejavuText {
    pub head: String,
    pub body: String,
    pub tail: String,
    pub range: Range<usize>,
}

pub enum DejavuTextTrim {
    /// `<%= ... =%>`
    Nothing,
    /// `<%~ ... ~%>`
    UntilLineBreak,
    /// `<%- ... -%>`
    RecentLineBreak,
    /// `<%_ ... _%>`
    FurthestLineBreak,
    /// `<%. ... .%>`
    AllLineBreaks,
}

impl Default for DejavuTextTrim {
    fn default() -> Self {
        Self::Nothing
    }
}

impl DejavuText {
    /// `... %>   \n  text`
    pub fn trim_head(&mut self, mode: DejavuTextTrim) {
        let mut offset = 0;
        let all = self.head.len();
        match mode {
            DejavuTextTrim::Nothing => {}
            DejavuTextTrim::UntilLineBreak => {
                for c in self.head.chars() {
                    match c {
                        '\r' | '\n' => break,
                        _ => offset += c.len_utf8(),
                    }
                }
                self.head = self.head.get(offset..all).unwrap().to_string()
            }
            DejavuTextTrim::RecentLineBreak => {
                let mut found = false;
                for c in self.head.chars() {
                    if found {
                        break;
                    }
                    match c {
                        '\r' | '\n' => {
                            offset += c.len_utf8();
                            found = true
                        }
                        _ => offset += c.len_utf8(),
                    }
                }
                self.head = self.head.get(offset..all).unwrap().to_string()
            }
            DejavuTextTrim::FurthestLineBreak => {
                let mut last = 0;
                for c in self.head.chars() {
                    match c {
                        '\r' | '\n' => {
                            offset += c.len_utf8();
                            last = offset
                        }
                        _ => offset += c.len_utf8(),
                    }
                }
                self.head = self.head.get(last..all).unwrap().to_string()
            }
            DejavuTextTrim::AllLineBreaks => self.head.clear(),
        }
    }
    /// `text  \n   <%`
    pub fn trim_tail(&mut self, mode: DejavuTextTrim) {
        let mut offset = self.tail.len();
        match mode {
            DejavuTextTrim::Nothing => {}
            DejavuTextTrim::UntilLineBreak => {
                for c in self.head.chars().rev() {
                    match c {
                        '\r' | '\n' => break,
                        _ => offset -= c.len_utf8(),
                    }
                }
                self.head = self.head.get(0..offset).unwrap().to_string()
            }
            DejavuTextTrim::RecentLineBreak => {
                let mut found = false;
                for c in self.head.chars() {
                    if found {
                        break;
                    }
                    match c {
                        '\r' | '\n' => {
                            offset += c.len_utf8();
                            found = true
                        }
                        _ => offset -= c.len_utf8(),
                    }
                }
                self.head = self.head.get(0..offset).unwrap().to_string()
            }
            DejavuTextTrim::FurthestLineBreak => {
                let mut last = 0;
                for c in self.head.chars() {
                    match c {
                        '\r' | '\n' => {
                            offset -= c.len_utf8();
                            last = offset
                        }
                        _ => offset -= c.len_utf8(),
                    }
                }
                self.head = self.head.get(0..last).unwrap().to_string()
            }
            DejavuTextTrim::AllLineBreaks => self.head.clear(),
        }
    }
}
