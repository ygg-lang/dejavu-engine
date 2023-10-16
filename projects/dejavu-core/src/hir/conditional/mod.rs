use super::*;

#[derive(Debug, Default)]
pub struct DejavuBranches {
    pub branches: Vec<DejavuConditional>,
    pub default: Option<DejavuConditional>,
}

#[derive(Debug, Default)]
pub struct DejavuConditional {
    pub condition: DejavuExpression,
    pub body: Vec<DejavuStatement>,
}

impl Display for DejavuBranches {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for (i, DejavuConditional { condition, body }) in self.branches.iter().enumerate() {
            if i == 0 {
                f.write_str("if ")?
            }
            else {
                f.write_str("else if ")?
            }
            f.write_fmt(format_args!("{}", condition))?;
            f.write_str(" {\n")?;
            for s in body {
                f.write_fmt(format_args!("{}", s))?;
            }
            f.write_str("}")?
        }
        match &self.default {
            Some(s) => {
                f.write_str("else ")?;
                f.write_fmt(format_args!("{}", s))?;
            }
            None => {}
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
