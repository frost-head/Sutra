use anyhow::Result;
use core::fmt;

use crate::{
    ast::{
        expression::{Expression, assign::Identifier, if_expr::parse_if},
        let_statement::LetStatement,
        return_statement::ReturnStatement,
        statement::Stmt,
    },
    errors::ParserError,
    lexer::token::{KeywordKind, OperatorKind, PuncuationKind, Token},
    parser::Parser,
    utils::indent_multiline,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    fn new(statements: Vec<Stmt>) -> Self {
        Block { statements }
    }

    pub fn parse(parser: &mut Parser) -> Result<Block> {
        let mut statements = Vec::new();
        loop {
            let next = parser.consume()?;
            match next {
                Token::Keyword(KeywordKind::Let) => {
                    let st = Stmt::LetStmt(LetStatement::parse(parser)?);
                    statements.push(st);
                }
                Token::Keyword(KeywordKind::Return) => {
                    let st = Stmt::ReturnStmt(ReturnStatement::parse(parser)?);
                    statements.push(st);
                }
                Token::Punctuation(PuncuationKind::RightCurlyParen) => {
                    return Ok(Block::new(statements));
                }
                Token::Punctuation(PuncuationKind::LeftCurlyParen) => {}
                Token::Keyword(KeywordKind::If) => {
                    let st = Stmt::Expr(parse_if(parser)?);
                    statements.push(st);
                }
                Token::Ident(identifier) => {
                    parser.expect(Token::Operator(OperatorKind::Equal))?;
                    let st = Stmt::Expr(Expression::Assign {
                        target: Identifier { name: identifier },
                        value: Box::new(Expression::parse_expression(parser, 0)?),
                    });
                    parser.expect(Token::Punctuation(PuncuationKind::SemiColon))?;
                    statements.push(st);
                }
                Token::EOF => {
                    break;
                }
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        token: next.clone(),
                    }
                    .into());
                }
            }
        }

        Ok(Block::new(statements))
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        for stmt in &self.statements {
            write!(f, "{}\n", indent_multiline(&stmt.to_string(), "    "))?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
