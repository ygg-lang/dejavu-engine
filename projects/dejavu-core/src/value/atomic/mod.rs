use super::*;

#[derive(Serialize, Deserialize)]
pub struct Namespace {
    pub path: Vec<Identifier>,
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}

impl Namespace {
    #[inline]
    pub fn new(path: Vec<Identifier>, span: &Range<usize>, file: &FileID) -> DjvNode {
        DjvNode { kind: DjvKind::Namespace(Namespace { path }), span: span.clone(), file: file.clone() }
    }
}

impl Debug for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, id) in self.path.iter().enumerate() {
            if idx != 0 {
                f.write_str("::")?
            }
            f.write_str(&id.name)?;
        }
        Ok(())
    }
}
