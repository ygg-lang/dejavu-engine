use dejavu_highlighter::DejavuHighlighter;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut renderer = DejavuHighlighter::default();
    renderer.render(&include_str!("hello_render.rs.djv"));
}
