use crate::lexer::token::OperatorKind;
use core::fmt;
mod parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal {
        literal: i64,
    },
    Binary {
        operator: OperatorKind,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Grouped {
        expression: Box<Expression>,
    },
    Unary {
        operator: OperatorKind,
        operand: Box<Expression>,
    },
    Ident {
        identifier: String,
    },
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal { literal } => write!(f, "{}", literal),
            Expression::Binary {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::Grouped { expression } => write!(f, "({})", expression),
            Expression::Unary { operator, operand } => write!(f, "({} {})", operator, operand),
            Expression::Ident { identifier } => write!(f, "{}", identifier),
        }
    }
}
