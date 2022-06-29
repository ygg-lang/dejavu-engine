use std::{fmt::Write, fs::File, io::Write as _};

use write_nodes::NodeWriter;

use crate::{parse, DjvNode};

use super::*;

mod write_nodes;

pub struct Compiler<'i> {
    config: &'i DejavuWorkspace,
    errors: Vec<QError>,
    output: PathBuf,
}

impl DejavuWorkspace {
    pub fn compile(&self, file: &FileID) -> QResult<Vec<QError>> {
        let mut c = Compiler { config: self, errors: vec![], output: Default::default() };
        c.compile(file)?;
        Ok(c.errors)
    }
}

impl Compiler<'_> {
    pub fn compile(&mut self, id: &FileID) -> QResult {
        let text = self.config.get_text(id)?;
        let nodes = parse(text, id).result(|e| self.errors.push(e))?;
        self.analyze(id, &nodes)?;
        let mut output = String::new();
        output.write_str(
            r#"
        
use core::fmt::Write;

use dejavu_runtime::{Result, Template};

impl Template for crate::hello::HelloTemplate {
    const SIZE_HINT: usize = 1024;
    const MIME_TYPE: &'static str = "text/rust; charset=utf-8";
    const EXTENSION: &'static str = "rust";

    fn render_into<W: Write + ?Sized>(&self, fmt: &mut W) -> Result<()> {
        "#,
        )?;

        for node in nodes {
            let mut w = NodeWriter { writer: &mut output, depth: 0, predefined_identifiers: &["None".to_string()] };
            node.write_nodes(&mut w)?;
        }
        output.write_str(
            r#"
        Ok(())
    }
}
        "#,
        )?;

        let mut file = File::create(&self.output)?;
        file.write_all(output.as_bytes())?;
        Ok(())
    }

    pub fn analyze(&mut self, id: &FileID, nodes: &[DjvNode]) -> QResult {
        let _ = nodes;
        let path: &Path = id.as_ref();
        self.output = path.with_extension("");
        Ok(())
    }
}
