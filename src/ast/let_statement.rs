use crate::errors::ParserError;
use crate::lexer::token::{KeywordKind, OperatorKind, PuncuationKind};
use crate::{ast::expression::Expresion, lexer::token::Token, parser::Parser};
use anyhow::Result;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct LetStatement {
    pub(crate) identifier: Token,
    pub(crate) value: Expresion,
}

impl LetStatement {
    pub fn new(identifier: Token, value: Expresion) -> LetStatement {
        LetStatement { identifier, value }
    }

    pub fn parse(parser: &mut Parser) -> Result<LetStatement> {
        let identifier: Token;
        let mut expression: Vec<Token> = Vec::new();
        if parser.tokens.peek().unwrap() == &Token::Keyword(KeywordKind::Let) {
            parser.tokens.next().unwrap();
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                got: parser.tokens.peek().unwrap().clone(),
                kind: Token::Keyword(KeywordKind::Let),
            }
            .into());
        }

        if let Token::Ident(_id) = parser.tokens.peek().unwrap() {
            identifier = parser.tokens.next().unwrap();
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                kind: Token::Ident("identifier".to_string()),
                got: parser.tokens.peek().unwrap().clone(),
            }
            .into());
        }

        if parser.tokens.peek().unwrap() == &Token::Operator(OperatorKind::Equal) {
            parser.tokens.next().unwrap();
        } else {
            return Err(ParserError::ExpectedTokenGotUnexpected {
                kind: Token::Operator(OperatorKind::Equal),
                got: parser.tokens.peek().unwrap().clone(),
            }
            .into());
        }

        loop {
            let cur = parser.tokens.next().unwrap();
            if cur == Token::Punctuation(PuncuationKind::SemiColon) {
                break;
            } else if cur == Token::EOF {
                return Err(ParserError::UnexpectedToken { token: cur }.into());
            } else {
                expression.push(cur);
            }
        }

        let statement = LetStatement::new(identifier, Expresion::new(expression));
        Ok(statement)
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Let Statement:\n  Identifier: {}\n  Value: {}",
            self.identifier, self.value
        )
    }
}
