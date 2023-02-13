use dejavu_highlighter::{ClassPalette, DejavuHighlighter};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut allocator = ClassPalette::new("");
    let mut renderer = DejavuHighlighter::default();
    renderer.render(&include_str!("hello_render.rs.djv"), &mut allocator).unwrap();
}
