use std::env::current_dir;

use peginator_codegen::Compile;

fn main() {
    let path = current_dir().unwrap();
    let output = path.join("src/saha.rs");
    Compile::file("src/saha.peg").destination(output).format().run().unwrap();
}
