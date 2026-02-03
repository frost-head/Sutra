use crate::{
    errors::{ParserError, span::Span},
    lexer::token::{KeywordKind, OperatorKind, PuncuationKind, TokenKind},
    parser::Parser,
    parser::ast::expression::{Expression, ExpressionKind, if_expr::parse_if},
};
use anyhow::{Context, Ok, Result};

impl Expression {
    pub fn parse(parser: &mut Parser) -> Result<Expression> {
        Self::parse_expression(parser, 0)
    }

    pub fn parse_prefix(parser: &mut Parser) -> Result<Expression> {
        let token = parser.peek()?.clone();

        match &token.kind {
            TokenKind::Number(num) => {
                parser.consume()?;
                Ok(Expression {
                    kind: ExpressionKind::Literal {
                        literal: num.clone(),
                    },
                    span: token.span,
                })
            }

            TokenKind::Ident(ident) => {
                parser.consume()?;
                Ok(Expression {
                    kind: ExpressionKind::Ident {
                        identifier: ident.clone(),
                    },
                    span: token.span,
                })
            }

            TokenKind::Keyword(KeywordKind::If) => parse_if(parser),

            TokenKind::Operator(OperatorKind::Bang) => {
                let tok = parser.consume()?; // consume `!`

                let rhs =
                    Self::parse_expression(parser, 8).context("Could not parse unary operand")?;

                Ok(Expression {
                    span: Span::merge(tok.span, rhs.span),
                    kind: ExpressionKind::Unary {
                        operator: OperatorKind::Bang,
                        operand: Box::new(rhs),
                    },
                })
            }

            TokenKind::Operator(OperatorKind::Minus) => {
                let tok = parser.consume()?; // consume `-`

                let rhs =
                    Self::parse_expression(parser, 8).context("Could not parse unary operand")?;

                Ok(Expression {
                    span: Span::merge(tok.span, rhs.span),
                    kind: ExpressionKind::Unary {
                        operator: OperatorKind::Minus, // ❗ FIXED
                        operand: Box::new(rhs),
                    },
                })
            }

            TokenKind::Punctuation(PuncuationKind::LeftParen) => {
                parser.consume()?; // consume '('

                let expr = Self::parse_expression(parser, 0)?;
                parser.expect(TokenKind::Punctuation(PuncuationKind::RightParen))?;
                Ok(expr)
            }

            _ => Err(ParserError::UnexpectedToken { token }.into()),
        }
    }

    pub fn parse_expression(parser: &mut Parser, min_bp: u8) -> Result<Expression> {
        let mut lhs = Self::parse_prefix(parser).context("Could not parse lhs")?;
        let mut span = lhs.clone().span;
        loop {
            let op = match &parser.peek()?.kind {
                TokenKind::Operator(operator_kind) => operator_kind,
                _ => break,
            };
            let oprator = op.clone();

            let (l_bp, r_bp) = Self::infix_binding_power(&op);
            if l_bp < min_bp {
                break;
            }
            parser.consume()?;
            let rhs = Self::parse_expression(parser, r_bp).context("Could not parse rhs")?;
            span = Span::merge(span, rhs.clone().span);
            lhs = Expression {
                kind: ExpressionKind::Binary {
                    left: Box::new(lhs),
                    operator: oprator,
                    right: Box::new(rhs),
                },
                span,
            };
        }
        Ok(lhs)
    }

    fn infix_binding_power(op: &&OperatorKind) -> (u8, u8) {
        match op {
            // Assignment (right associative)
            OperatorKind::Equal => (1, 1),

            // Comparisons (non-associative)
            OperatorKind::EqualEqual | OperatorKind::Less | OperatorKind::Greater => (2, 3),

            // Addition / subtraction
            OperatorKind::Plus | OperatorKind::Minus => (4, 5),

            // Multiplication / division / modulo
            OperatorKind::Star | OperatorKind::Slash | OperatorKind::Percent => (6, 7),

            // Everything else MUST NOT bind
            _ => (0, 0),
        }
    }
}
