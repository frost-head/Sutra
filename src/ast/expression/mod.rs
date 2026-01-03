use crate::{
    ast::{block::Block, expression::assign::Identifier},
    lexer::token::OperatorKind,
};
use core::fmt;
pub mod assign;
pub mod if_expr;
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
    If {
        if_expr: Box<Expression>,
        then_block: Box<Expression>,
        else_block: Option<Box<Expression>>,
    },
    Block(Block),
    Assign {
        target: Identifier,
        value: Box<Expression>,
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
            Expression::If {
                if_expr,
                then_block,
                else_block,
            } => {
                write!(
                    f,
                    "if : {} then {}{}",
                    if_expr,
                    then_block,
                    match else_block {
                        Some(else_blk) => format!(" else : {}", else_blk),
                        None => String::new(),
                    }
                )
            }
            Expression::Block(block) => write!(f, "{}", block),
            Expression::Assign { target, value } => write!(f, "ident {} binds {}", target, value),
        }
    }
}
