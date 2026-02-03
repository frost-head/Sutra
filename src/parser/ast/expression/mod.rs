use crate::{
    errors::span::Span,
    lexer::token::OperatorKind,
    parser::ast::{block::Block, expression::assign::Identifier},
};
use core::fmt;
pub mod assign;
pub mod if_expr;
mod parse;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
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
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionKind::Literal { literal } => write!(f, "{}", literal),
            ExpressionKind::Binary {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            ExpressionKind::Grouped { expression } => write!(f, "({})", expression),
            ExpressionKind::Unary { operator, operand } => write!(f, "({} {})", operator, operand),
            ExpressionKind::Ident { identifier } => write!(f, "{}", identifier),
            ExpressionKind::If {
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
            ExpressionKind::Block(block) => write!(f, "{}", block),
            ExpressionKind::Assign { target, value } => {
                write!(f, "ident {} binds {}", target, value)
            }
        }
    }
}
