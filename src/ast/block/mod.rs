use anyhow::Result;
use core::fmt;

use crate::{
    ast::{
        expression::{Expression, ExpressionKind, assign::Identifier, if_expr::parse_if},
        let_statement::LetStatement,
        return_statement::ReturnStatement,
        statement::Stmt,
    },
    errors::{ParserError, span::Span},
    lexer::token::{KeywordKind, OperatorKind, PuncuationKind, TokenKind},
    parser::Parser,
    utils::indent_multiline,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

impl Block {
    fn new(statements: Vec<Stmt>, span: Span) -> Self {
        Block { statements, span }
    }

    pub fn parse(parser: &mut Parser) -> Result<Block> {
        let mut statements = Vec::new();
        let span = parser.peek()?.span;
        loop {
            let next = parser.peek()?;
            match &next.kind {
                TokenKind::Keyword(KeywordKind::Let) => {
                    let st = Stmt::LetStmt(LetStatement::parse(parser)?);
                    statements.push(st);
                }
                TokenKind::Keyword(KeywordKind::Return) => {
                    let st = Stmt::ReturnStmt(ReturnStatement::parse(parser)?);
                    statements.push(st);
                }
                TokenKind::Punctuation(PuncuationKind::RightCurlyParen) => {
                    let last = parser.consume()?;
                    return Ok(Block::new(statements, Span::merge(span, last.span)));
                }
                TokenKind::Punctuation(PuncuationKind::LeftCurlyParen) => {
                    parser.consume()?;
                }
                TokenKind::Keyword(KeywordKind::If) => {
                    let st = Stmt::Expr(parse_if(parser)?);
                    statements.push(st);
                }
                TokenKind::Ident(_) => {
                    let st = parse_ident(parser)?;
                    statements.push(st);
                }
                TokenKind::EOF => {
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

        Ok(Block::new(statements, span))
    }
}

fn parse_ident(parser: &mut Parser<'_>) -> Result<Stmt, anyhow::Error> {
    let identifier: String;
    let ident = parser.consume()?;
    if let TokenKind::Ident(id) = ident.kind {
        identifier = id.clone();
    } else {
        return Err(ParserError::ExpectedTokenGotUnexpected {
            kind: TokenKind::Ident("identifier".to_string()),
            got: parser.peek()?.clone(),
        }
        .into());
    }
    parser.expect(TokenKind::Operator(OperatorKind::Equal))?;
    let value = Box::new(Expression::parse_expression(parser, 0)?);
    let semicolon = parser.expect(TokenKind::Punctuation(PuncuationKind::SemiColon))?;

    let st = Stmt::Expr(Expression {
        kind: ExpressionKind::Assign {
            target: Identifier { name: identifier },
            value,
        },
        span: Span::merge(ident.span, semicolon.span),
    });
    Ok(st)
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
