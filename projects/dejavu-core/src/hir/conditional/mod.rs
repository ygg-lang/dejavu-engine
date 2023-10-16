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

impl AddAssign<DejavuConditional> for DejavuBranches {
    fn add_assign(&mut self, rhs: DejavuConditional) {
        self.branches.push(rhs)
    }
}
