use yggdrasil_shared::codegen::RustCodegen;

fn main() {
    let grammars = std::path::Path::new("grammars/").canonicalize().unwrap();
    let builder = RustCodegen::default();
    builder.generate(include_str!("grammars/dejav.ygg"), "src/dejav.ygg").unwrap();
    println!("cargo:rerun-if-changed={}", grammars.display());
}
