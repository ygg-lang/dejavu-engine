use peginator::buildscript::Compile;
use std::env::current_dir;

fn main() {
    let path = current_dir().unwrap();
    let output = path.join("src/parser/saha.rs");
    Compile::file("src/parser/saha.peg").destination(output).format().run().unwrap();
}
