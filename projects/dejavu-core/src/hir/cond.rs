use super::*;

#[derive(Clone, Debug, Default)]
pub struct DejavuBranches {
    pub branches: Vec<DejavuConditional>,
    pub default: DejavuSequence,
}

#[derive(Clone, Debug, Default)]
pub struct DejavuConditional {
    pub condition: DejavuExpression,
    pub body: DejavuSequence,
}

impl DisplayIndent for DejavuBranches {
    fn fmt_indent<W: Write>(&self, f: &mut IndentFormatter<W>) -> core::fmt::Result {
        for (i, c) in self.branches.iter().enumerate() {
            if i == 0 {
                f.write_str("if ")?
            }
            else {
                f.write_str(" else if ")?
            }
            c.fmt_indent(f)?;
        }
        if !self.default.is_empty() {
            f.write_str(" else ")?;
            self.default.fmt_indent(f)?;
        }
        Ok(())
    }
}

impl DisplayIndent for DejavuConditional {
    fn fmt_indent<W: Write>(&self, f: &mut IndentFormatter<W>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.condition))?;
        f.write_str(" ")?;
        self.body.fmt_indent(f)?;
        Ok(())
    }
}

impl AddAssign<DejavuConditional> for DejavuBranches {
    fn add_assign(&mut self, rhs: DejavuConditional) {
        self.branches.push(rhs)
    }
}

impl DejavuBranches {
    pub fn new(capacity: usize) -> Self {
        Self { branches: Vec::with_capacity(capacity), default: Default::default() }
    }
}
