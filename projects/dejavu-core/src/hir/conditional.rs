use super::*;

#[derive(Clone, Debug, Default)]
pub struct DejavuBranches {
    pub branches: Vec<DejavuConditional>,
    pub default: Vec<DejavuStatement>,
}

#[derive(Clone, Debug, Default)]
pub struct DejavuConditional {
    pub condition: DejavuExpression,
    pub body: Vec<DejavuStatement>,
}

impl DisplayIndent for DejavuBranches {
    fn fmt_indent<W: Write>(&self, mut f: IndentFormatter<W>) -> core::fmt::Result {
        for (i, c) in self.branches.iter().enumerate() {
            if i == 0 {
                f.write_str("if ")?
            }
            else {
                f.write_str("else if ")?
            }
            f.write_fmt(format_args!("{}", c))?;
            // f.write_str(" {\n")?;
            //
            // f.write_str("}\n")?;
        }
        if !self.default.is_empty() {
            f.write_str("else {")?;
            f.indent();
            f.write_newline()?;
            for rest in &self.default {
                f.write_fmt(format_args!("{}", rest))?;
            }
            f.dedent();
            f.write_str("}")?;
        }
        Ok(())
    }
}

impl DisplayIndent for DejavuConditional {
    fn fmt_indent<W: Write>(&self, mut f: IndentFormatter<W>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.condition))?;
        f.write_str(" {")?;
        f.indent();
        f.write_newline()?;
        for s in &self.body {
            f.write_fmt(format_args!("{}", s))?;
        }
        f.dedent();
        f.write_str("}")
    }
}

impl AddAssign<DejavuConditional> for DejavuBranches {
    fn add_assign(&mut self, rhs: DejavuConditional) {
        self.branches.push(rhs)
    }
}

impl DejavuBranches {
    pub fn new(capacity: usize) -> Self {
        Self { branches: Vec::with_capacity(capacity), default: Vec::new() }
    }
}
