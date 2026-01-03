use crate::{
    ast::{block::Block, expression::Expression},
    lexer::token::{KeywordKind, Token},
    parser::Parser,
};
use anyhow::{Ok, Result};

pub fn parse_if(parser: &mut Parser) -> Result<Expression> {
    let expr = Expression::parse_expression(parser, 0)?;

    let then_block = Block::parse(parser)?;

    let mut else_block = None;

    if parser.peek()? == &Token::Keyword(KeywordKind::Else) {
        parser.consume()?;
        else_block = Some(Block::parse(parser)?);
    }

    match else_block {
        Some(else_block) => Ok(Expression::If {
            if_expr: Box::new(expr),
            then_block: Box::new(Expression::Block(then_block)),
            else_block: Some(Box::new(Expression::Block(else_block))),
        }),
        None => Ok(Expression::If {
            if_expr: Box::new(expr),
            then_block: Box::new(Expression::Block(then_block)),
            else_block: None,
        }),
    }
}
