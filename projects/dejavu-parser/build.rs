use yggdrasil_shared::codegen::RustCodegen;

fn main() {
    let grammars = std::path::Path::new("grammars/").canonicalize().unwrap();
    let builder = RustCodegen::default();
    builder.generate(include_str!("grammars/dejavu.ygg"), "src/dejavu").unwrap();
    println!("cargo:rerun-if-changed={}", grammars.display());
}
