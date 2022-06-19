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
        for node in nodes {
            write!(output, "{:o}", node)?;
        }
        Ok(())
    }
    pub fn analyze(&mut self, id: &FileID, nodes: &[DjvNode]) -> QResult {
        let _ = nodes;
        let path: &Path = id.as_ref();
        self.output = path.with_extension("");
        Ok(())
    }
}
