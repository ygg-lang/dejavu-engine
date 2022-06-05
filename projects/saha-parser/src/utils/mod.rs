use ucd_trie::TrieSetOwned;

pub fn unicode_whitespace(input: &str) -> Result<(&str, usize), &'static str> {
    let mut length = 0;
    for char in input.chars() {
        if char.is_whitespace() { length += char.len_utf8() } else { break }
    }
    if length == 0 { Err("Not unicode whitespace") } else { Ok((&input[0..length], length)) }
}

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

pub fn maybe_number(input: &str) -> Result<(&str, usize), &'static str> {
    let mut length = 0;
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            c if c.is_numeric() => length += c.len_utf8(),
            '*' => match chars.next() {
                Some('*') => length += 2,
                _ => break,
            },
            _ => break,
        }
    }
    if length == 0 { Err("Not number text") } else { Ok((&input[0..length], length)) }
}
