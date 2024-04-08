use dejavu_runtime::escaper::Html;
use std::string::ToString;

#[test]
fn test_escape() {
    assert_eq!(escape("", Html).to_string(), "");
    assert_eq!(escape("<&>", Html).to_string(), "&lt;&amp;&gt;");
    assert_eq!(escape("bla&", Html).to_string(), "bla&amp;");
    assert_eq!(escape("<foo", Html).to_string(), "&lt;foo");
    assert_eq!(escape("bla&h", Html).to_string(), "bla&amp;h");
}
