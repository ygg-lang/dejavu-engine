use crate::dejavu::{ElementNode, RootNode, TextElementsNode};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct DejavuDisplay {
    pub root: RootNode,
}

impl Display for DejavuDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "impl core::fmt::Display for HelloTemplate {{")?;
        writeln!(f, "#[inline]")?;
        writeln!(f, "fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {{")?;
        for x in &self.root.element {
            match x {
                ElementNode::TemplateExport(_) => {}
                ElementNode::TemplateIf(_) => {}
                ElementNode::TextElements(x) => match x {
                    TextElementsNode::TemplateE(_) => writeln!(f, "f.write_str(\"<%\")?")?,
                    TextElementsNode::TextSpace(s) => writeln!(f, "f.write_str(\"{s:?}\")?;")?,
                    TextElementsNode::TextWord(w) => writeln!(f, "f.write_str(\"{w:?}\")?;")?,
                },
            }
        }
        writeln!(f, "Ok(())}}}}")?;
        Ok(())
    }
}
