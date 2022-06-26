use super::*;

impl BinaryOperator {
    pub fn as_precedence(&self) -> Precedence {
        let o = match self {
            BinaryOperator::Addition => 10,
            BinaryOperator::Subtraction => 10,
            BinaryOperator::Multiplication => 20,
            BinaryOperator::Division => 20,
            BinaryOperator::Power => 20,
            BinaryOperator::Assign => 5,
        };
        Precedence(o)
    }
    pub fn as_associativity(&self) -> Associativity {
        match self {
            BinaryOperator::Addition => Associativity::Left,
            BinaryOperator::Subtraction => Associativity::Left,
            BinaryOperator::Multiplication => Associativity::Left,
            BinaryOperator::Division => Associativity::Left,
            BinaryOperator::Power => Associativity::Right,
            BinaryOperator::Assign => Associativity::Neither,
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinaryOperator::Addition => "+",
            BinaryOperator::Subtraction => "-",
            BinaryOperator::Multiplication => "*",
            BinaryOperator::Division => "/",
            BinaryOperator::Power => "^",
            BinaryOperator::Assign => "=",
        };
        write!(f, "{}", s)
    }
}
