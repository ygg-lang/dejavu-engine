pub fn unicode_whitespace(input: &str) -> Result<(&str, usize), &'static str> {
    let mut length = 0;
    for char in input.chars() {
        if char.is_whitespace() { length += char.len_utf8() } else { break }
    }
    if length == 0 { return Err("No unicode whitespace") } else { Ok((&input[0..length], length)) }
}
