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

impl Display for DejavuBranches {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
            f.write_str("else {\n")?;
            for rest in &self.default {
                f.write_fmt(format_args!("{}", rest))?;
            }
            f.write_str("}\n")?;
        }
        Ok(())
    }
}

impl Display for DejavuConditional {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.condition))?;
        f.write_str(" {\n")?;
        for s in &self.body {
            f.write_fmt(format_args!("{}", s))?;
        }
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
