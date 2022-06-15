use std::string::ToString;

use dejavu_engine::{escape, XmlEscaper};

#[test]
fn test_escape() {
    assert_eq!(escape("", XmlEscaper).to_string(), "");
    assert_eq!(escape("<&>", XmlEscaper).to_string(), "&lt;&amp;&gt;");
    assert_eq!(escape("bla&", XmlEscaper).to_string(), "bla&amp;");
    assert_eq!(escape("<foo", XmlEscaper).to_string(), "&lt;foo");
    assert_eq!(escape("bla&h", XmlEscaper).to_string(), "bla&amp;h");
}
