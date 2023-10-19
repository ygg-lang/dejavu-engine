use core::fmt::Write;

use indentation::{DisplayIndent, IndentFormatter};

use crate::hir::{DejavuExpression, DejavuSequence};

/// ```dejavu
/// <% for i in j if k %>
///
/// <% else %>
///
/// <% end %>
/// ```
#[derive(Clone, Debug)]
pub struct DejavuForLoop {
    pub pattern: DejavuPattern,
    pub iterator: DejavuExpression,
    pub condition: Option<DejavuExpression>,
    pub body: DejavuSequence,
    pub otherwise: Option<DejavuSequence>,
}

#[derive(Clone, Debug)]
pub enum DejavuPattern {}

impl DisplayIndent for DejavuForLoop {
    /// ```dejavu
    /// let mut _looped = false;
    /// for i in j {
    ///     if !k {
    ///         continue;
    ///     }
    /// }
    /// if !_looped {}
    /// ```
    fn fmt_indent<W: Write>(&self, f: &mut IndentFormatter<W>) -> core::fmt::Result {
        if let Some(_) = &self.otherwise {
            f.write_str("let mut _looped = false;")?;
        }
        f.write_str("for i in j {")?;
        match &self.condition {
            Some(s) => {
                f.write_str("if !k")?;

                f.write_str("continue;")?;
                f.write_str("}")?;
            }
            None => {}
        }
        if let Some(_) = &self.otherwise {
            f.write_str("_looped = true;")?;
        }
        for e in &self.body.statements {
            e.fmt_indent(f)?;
        }
        f.write_str("}")?;
        if let Some(o) = &self.otherwise {
            f.write_str("if !_looped ")?;
            o.fmt_indent(f)?;
        }

        Ok(())
    }
}
