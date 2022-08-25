use super::*;

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self { hint: "".to_string(), text: value.to_string() }
    }
}
impl From<&String> for Text {
    fn from(value: &String) -> Self {
        Self { hint: "".to_string(), text: value.to_string() }
    }
}
impl From<String> for Text {
    fn from(value: String) -> Self {
        Self { hint: "".to_string(), text: value }
    }
}
