use crate::{
    ast::{
        block::Block,
        expression::{Expression, ExpressionKind},
    },
    errors::span::Span,
    lexer::token::{KeywordKind, TokenKind},
    parser::Parser,
};
use anyhow::{Ok, Result};

pub fn parse_if(parser: &mut Parser) -> Result<Expression> {
    let if_expr = parser.expect(TokenKind::Keyword(KeywordKind::If))?;
    let expr = Expression::parse_expression(parser, 0)?;

    let then_block = Block::parse(parser)?;

    let mut else_block = None;

    let mut span = Span::merge(if_expr.span, then_block.span);
    if parser.peek()?.kind == TokenKind::Keyword(KeywordKind::Else) {
        parser.consume()?;
        let block = Block::parse(parser)?;
        else_block = Some(block.clone());
        span = Span::merge(span, block.span);
    }

    match else_block {
        Some(else_block) => Ok(Expression {
            kind: ExpressionKind::If {
                if_expr: Box::new(expr),
                then_block: Box::new(Expression {
                    kind: ExpressionKind::Block(then_block.clone()),
                    span: then_block.span,
                }),
                else_block: Some(Box::new(Expression {
                    kind: ExpressionKind::Block(else_block.clone()),
                    span: else_block.span,
                })),
            },
            span: span,
        }),
        None => Ok(Expression {
            kind: ExpressionKind::If {
                if_expr: Box::new(expr),
                then_block: Box::new(Expression {
                    kind: ExpressionKind::Block(then_block.clone()),
                    span: then_block.span,
                }),
                else_block: None,
            },
            span: span,
        }),
    }
}
