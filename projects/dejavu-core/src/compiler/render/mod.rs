use std::{fs::File, io::Write};

use diagnostic_quick::Validation;

use crate::{parse, DjvNode};

use super::*;

pub struct Compiler<'i> {
    config: &'i DejavuWorkspace,
    errors: Vec<QError>,
    output: PathBuf,
}

impl DejavuWorkspace {
    pub fn compile_all(&self) -> Validation<String> {
        todo!()
    }
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
        let mut output = File::create(&self.output)?;
        output.write_all(
            r#"
        
use core::fmt::Write;

use dejavu_engine::{Result, Template};

impl Template for crate::hello::HelloTemplate {
    const SIZE_HINT: usize = 100;
    const MIME_TYPE: &'static str = "text/html; charset=utf-8";
    const EXTENSION: &'static str = "html";

    fn render_into<W: Write + ?Sized>(&self, fmt: &mut W) -> Result<()> {
        "#
            .as_bytes(),
        )?;

        for node in nodes {
            write!(output, "{:o}", node)?;
        }
        output.write_all(
            r#"
        Ok(())
    }
}
        "#
            .as_bytes(),
        )?;
        Ok(())
    }

    pub fn analyze(&mut self, id: &FileID, nodes: &[DjvNode]) -> QResult {
        let _ = nodes;
        let path: &Path = id.as_ref();
        self.output = path.with_extension("");
        Ok(())
    }
}
