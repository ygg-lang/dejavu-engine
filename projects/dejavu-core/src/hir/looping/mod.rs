use super::*;

/// ```dejavu
/// <% for i in j if k %>
///
/// <% else %>
///
/// <% end %>
/// ```
#[derive(Clone, Debug)]
pub struct DejavuLoop {
    pub pattern: DejavuPattern,
    pub iterator: DejavuExpression,
    pub condition: Option<DejavuExpression>,
    pub body: DejavuSequence,
    pub otherwise: Option<DejavuSequence>,
}

#[derive(Clone, Debug)]
pub enum DejavuPattern {
    Bare(Vec<DejavuIdentifier>),
}

impl DisplayIndent for DejavuLoop {
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
            f.write_newline()?;
        }
        f.write_str("for i in j {")?;
        f.indent();
        f.write_newline()?;
        match &self.condition {
            Some(s) => {
                f.write_str("if !(")?;
                write!(f, "{}", s)?;
                f.write_str(") {")?;
                f.indent();
                f.write_newline()?;
                f.write_str("continue;")?;
                f.dedent();
                f.write_newline()?;
                f.write_str("}")?;
                f.write_newline()?;
            }
            None => {}
        }
        if let Some(_) = &self.otherwise {
            f.write_str("_looped = true;")?;
            f.write_newline()?;
        }
        for e in &self.body.statements {
            e.fmt_indent(f)?;
        }
        f.dedent();
        f.write_newline()?;
        f.write_str("}")?;
        if let Some(o) = &self.otherwise {
            f.write_newline()?;
            f.write_str("if !_looped ")?;
            o.fmt_indent(f)?;
        }
        f.write_newline()?;
        Ok(())
    }
}

impl DejavuLoop {
    pub fn new(pattern: DejavuPattern) -> Self {
        Self { pattern, iterator: Default::default(), condition: None, body: Default::default(), otherwise: None }
    }
}
